use super::handle;
use crate::{errors, event_loop::Webhook};
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::time::timeout;
use tracing::instrument;

/// Configures the HTTP webhook server.
#[must_use = "webhook server needs to be `start`ed to run the event loop"]
pub struct Http<'a> {
    webhook: Webhook<'a>,
}

impl<'a> Http<'a> {
    pub(crate) const fn new(webhook: Webhook<'a>) -> Self {
        Self { webhook }
    }
}

impl<'a> Http<'a> {
    /// Starts the server.
    #[instrument(name = "http_webhook", skip(self))]
    pub async fn start(self) -> Result<Infallible, errors::HttpWebhook> {
        let Webhook {
            event_loop,
            ip,
            port,
            updates_url,
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

        let set_commands = event_loop.set_commands_descriptions();
        match timeout(request_timeout, set_commands).await {
            Ok(Err(method)) => {
                return Err(errors::HttpWebhook::SetMyCommands(method))
            }
            Err(timeout) => {
                return Err(errors::HttpWebhook::SetMyCommandsTimeout(timeout))
            }
            Ok(_) => (),
        };

        let event_loop = Arc::new(event_loop);
        let addr = SocketAddr::new(ip, port);
        let updates_url = Arc::new(updates_url);

        Server::bind(&addr)
            .serve(make_service_fn(move |_| {
                let event_loop = Arc::clone(&event_loop);
                let updates_url = Arc::clone(&updates_url);

                async move {
                    let service = service_fn(move |request| {
                        handle(
                            Arc::clone(&event_loop),
                            request,
                            Arc::clone(&updates_url),
                        )
                    });

                    Ok::<_, hyper::Error>(service)
                }
            }))
            .await?;

        unreachable!("[tbot] The webhook server unexpectedly returned.");
    }
}
