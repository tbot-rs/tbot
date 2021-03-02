//! Types related to the HTTPS webhook server.

use super::handle;
use crate::{errors, event_loop::Webhook};
use hyper::{server::conn::Http, service::service_fn};
use hyper::{Body, Request};
use tracing::instrument;

#[cfg(feature = "tls")]
pub use native_tls::Identity;
#[cfg(feature = "tls")]
use native_tls::TlsAcceptor;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::time::timeout;
#[cfg(feature = "rustls")]
pub use tokio_rustls::rustls::ServerConfig;
#[cfg(feature = "rustls")]
use tokio_rustls::TlsAcceptor;

/// Configures the HTTPS webhook server.
#[must_use = "webhook server needs to be `start`ed to run the event loop"]
pub struct Https<'a> {
    webhook: Webhook<'a>,
    #[cfg(feature = "tls")]
    identity: Identity,
    #[cfg(feature = "rustls")]
    config: ServerConfig,
}

impl<'a> Https<'a> {
    pub(crate) const fn new(
        webhook: Webhook<'a>,
        #[cfg(feature = "tls")] identity: Identity,
        #[cfg(feature = "rustls")] config: ServerConfig,
    ) -> Self {
        Self {
            webhook,
            #[cfg(feature = "tls")]
            identity,
            #[cfg(feature = "rustls")]
            config,
        }
    }
}

impl<'a> Https<'a> {
    /// Starts the event loop.
    #[instrument(name = "https_webhook", skip(self))]
    pub async fn start(self) -> Result<Infallible, errors::HttpsWebhook> {
        let Webhook {
            event_loop,
            bind_to,
            port,
            updates_url,
            url,
            ip_address,
            certificate,
            max_connections,
            allowed_updates,
            request_timeout,
        } = self.webhook;

        let set_webhook = event_loop
            .bot
            .set_webhook(
                url,
                ip_address,
                certificate,
                max_connections,
                allowed_updates,
            )
            .call();

        timeout(request_timeout, set_webhook).await??;

        let set_commands = event_loop.set_commands_descriptions();
        match timeout(request_timeout, set_commands).await {
            Ok(Err(method)) => {
                return Err(errors::HttpsWebhook::SetMyCommands(method))
            }
            Err(timeout) => {
                return Err(errors::HttpsWebhook::SetMyCommandsTimeout(timeout))
            }
            Ok(_) => (),
        };

        let event_loop = Arc::new(event_loop);
        let addr = SocketAddr::new(bind_to, port);
        let updates_url = Arc::new(updates_url);

        #[cfg(feature = "tls")]
        let tls_acceptor = {
            let tls_acceptor = TlsAcceptor::builder(self.identity).build()?;
            tokio_native_tls::TlsAcceptor::from(tls_acceptor)
        };
        #[cfg(feature = "rustls")]
        let tls_acceptor = TlsAcceptor::from(Arc::new(self.config));

        let server = TcpListener::bind(&addr).await?;

        let http_proto = Http::new();

        loop {
            let (tcp_stream, _) = server.accept().await?;
            let tls_stream = tls_acceptor.accept(tcp_stream).await?;

            let event_loop = Arc::clone(&event_loop);
            let updates_url = Arc::clone(&updates_url);

            let service = service_fn(move |request: Request<Body>| {
                handle(
                    Arc::clone(&event_loop),
                    request,
                    Arc::clone(&updates_url),
                )
            });

            let conn = http_proto.serve_connection(tls_stream, service);

            conn.await?;
        }
    }
}
