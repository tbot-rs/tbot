use super::handle;
use crate::{connectors::Connector, errors, event_loop::Webhook};
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::time::timeout;
use tracing::instrument;

/// Configures the HTTP webhook server.
#[must_use = "webhook server needs to be `start`ed to run the event loop"]
pub struct Http<'a, C> {
    webhook: Webhook<'a, C>,
}

impl<'a, C> Http<'a, C> {
    pub(crate) const fn new(webhook: Webhook<'a, C>) -> Self {
        Self { webhook }
    }
}

impl<'a, C: Connector + Clone> Http<'a, C> {
    /// Starts the server.
    #[instrument(name = "http_webhook", skip(self))]
    pub async fn start(self) -> Result<Infallible, errors::HttpWebhook> {
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

        Server::bind(&addr)
            .serve(make_service_fn(move |_| {
                let bot = Arc::clone(&bot);
                let event_loop = Arc::clone(&event_loop);

                async {
                    let service = service_fn(move |request| {
                        handle(
                            Arc::clone(&bot),
                            Arc::clone(&event_loop),
                            request,
                        )
                    });

                    Ok::<_, hyper::Error>(service)
                }
            }))
            .await?;

        unreachable!("[tbot] The webhook server unexpectedly returned.");
    }
}
