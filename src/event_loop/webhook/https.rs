//! Types related to the HTTPS webhook server.

use super::handle;
use crate::{
    errors,
    event_loop::{EventLoop, Webhook},
    internal::BoxFuture,
};
use futures::{
    future::{self, Either},
    Future, IntoFuture, Stream,
};
use hyper::{
    client::connect::Connect, server::conn::Http, service::service_fn,
};
use native_tls::TlsAcceptor;
use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tokio::util::FutureExt;

pub use native_tls::Identity;

/// Configures the HTTPS webhook server.
pub struct Https<'a, C> {
    webhook: Webhook<'a, C>,
    identity: Identity,
}

impl<'a, C> Https<'a, C> {
    pub(crate) const fn new(webhook: Webhook<'a, C>, identity: Identity) -> Self {
        Self { webhook, identity }
    }
}

impl<'a, C> Https<'a, C>
where
    C: Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    /// Starts the server.
    pub fn start(self) -> ! {
        crate::run(self.into_future().map_err(|err| {
            eprintln!("[tbot] Webhook error: {:#?}", err);
        }));

        unreachable!(
            "[tbot] The webhook server unexpectedly returned. \
             An error should be printed above.",
        );
    }
}

impl<'a, C> IntoFuture for Https<'a, C>
where
    C: Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::HttpsWebhook;

    fn into_future(self) -> Self::Future {
        let set_webhook = self
            .webhook
            .event_loop
            .bot
            .set_webhook(
                self.webhook.url,
                self.webhook.certificate,
                self.webhook.max_connections,
                self.webhook.allowed_updates,
            )
            .into_future()
            .timeout(self.webhook.request_timeout)
            .map_err(errors::HttpsWebhook::SetWebhook);

        let Webhook {
            event_loop,
            ip,
            port,
            ..
        } = self.webhook;

        let identity = self.identity;

        Box::new(
            set_webhook
                .and_then(move |_| init_server(event_loop, ip, port, identity)),
        )
    }
}

fn init_server<C>(
    event_loop: EventLoop<C>,
    ip: IpAddr,
    port: u16,
    identity: Identity,
) -> impl Future<Item = (), Error = errors::HttpsWebhook>
where
    C: Clone + Send + Sync + 'static,
{
    let bot = Arc::new(event_loop.bot.clone());
    let event_loop = Arc::new(event_loop);
    let addr = SocketAddr::new(ip, port);

    let tls_acceptor = match TlsAcceptor::builder(identity).build() {
        Ok(tls_acceptor) => tls_acceptor,
        Err(err) => {
            return Either::A(future::err(errors::HttpsWebhook::Tls(err)))
        }
    };
    let tls_acceptor = tokio_tls::TlsAcceptor::from(tls_acceptor);

    let service = move || {
        let bot = Arc::clone(&bot);
        let event_loop = Arc::clone(&event_loop);
        service_fn(move |request| {
            handle(Arc::clone(&bot), Arc::clone(&event_loop), request)
        })
    };

    let server = match TcpListener::bind(&addr) {
        Ok(server) => server,
        Err(error) => {
            return Either::A(future::err(errors::HttpsWebhook::Bind(error)));
        }
    };

    let http_proto = Http::new();
    let http_server = http_proto
        .serve_incoming(
            server
                .incoming()
                .map_err(|error| {
                    let boxed: Box<dyn Error + Send + Sync> = Box::new(error);
                    boxed
                })
                .and_then(move |socket| {
                    tls_acceptor.accept(socket).map_err(|error| {
                        let boxed: Box<dyn Error + Send + Sync> =
                            Box::new(error);
                        boxed
                    })
                }),
            service,
        )
        .map_err(errors::HttpsWebhook::Server)
        .for_each(|conn| {
            crate::spawn(
                conn.and_then(|c| c.map_err(|e| panic!("Hyper error {}", e)))
                    .map_err(|e| eprintln!("Connection error {}", e)),
            );
            Ok(())
        });

    Either::B(http_server)
}
