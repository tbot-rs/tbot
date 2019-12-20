//! Types related to the HTTPS webhook server.

use super::handle;
use crate::{connectors::Connector, errors, event_loop::Webhook};
use hyper::{server::conn::Http, service::service_fn};
use hyper::{Body, Request};
use native_tls::TlsAcceptor;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::time::timeout;

pub use native_tls::Identity;

/// Configures the HTTPS webhook server.
#[must_use = "webhook server needs to be `start`ed to run the event loop"]
pub struct Https<'a, C> {
    webhook: Webhook<'a, C>,
    identity: Identity,
}

impl<'a, C> Https<'a, C> {
    pub(crate) const fn new(
        webhook: Webhook<'a, C>,
        identity: Identity,
    ) -> Self {
        Self { webhook, identity }
    }
}

impl<'a, C: Connector + Clone> Https<'a, C> {
    /// Starts the event loop.
    pub async fn start(self) -> Result<Infallible, errors::HttpsWebhook> {
        let Webhook {
            event_loop,
            ip,
            port,
            url,
            certificate,
            max_connections,
            allowed_updates,
            request_timeout,
        } = self.webhook;

        let set_webhook = event_loop
            .bot
            .set_webhook(url, certificate, max_connections, allowed_updates)
            .call();

        timeout(request_timeout, set_webhook).await??;

        let bot = Arc::new(event_loop.bot.clone());
        let event_loop = Arc::new(event_loop);
        let addr = SocketAddr::new(ip, port);

        let tls_acceptor = TlsAcceptor::builder(self.identity).build()?;
        let tls_acceptor = tokio_tls::TlsAcceptor::from(tls_acceptor);

        let mut server = TcpListener::bind(&addr).await?;

        let http_proto = Http::new();

        loop {
            let (tcp_stream, _) = server.accept().await?;
            let tls_stream = tls_acceptor.accept(tcp_stream).await?;

            let bot = Arc::clone(&bot);
            let event_loop = Arc::clone(&event_loop);

            let service = service_fn(move |request: Request<Body>| {
                handle(Arc::clone(&bot), Arc::clone(&event_loop), request)
            });

            let conn = http_proto.serve_connection(tls_stream, service);

            conn.await?;
        }
    }
}
