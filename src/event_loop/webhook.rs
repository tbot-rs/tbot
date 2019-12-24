//! Types related to the webhook event loop.

use super::EventLoop;
use crate::{types::parameters::UpdateKind, Bot};
use hyper::{
    body::{Body, HttpBody},
    Method, Request, Response,
};
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
    time::Duration,
};

mod http;
pub mod https;

pub use http::Http;
pub use https::Https;

/// Configures webhook and starts a server.
///
/// To construct a `Webhook`, use [`Bot::webhook`].
///
/// [`Bot::webhook`]: ./struct.Bot.html#method.webhook
#[must_use = "webhook does not start unless `start` is called"]
pub struct Webhook<'a, C> {
    event_loop: EventLoop<C>,
    ip: IpAddr,
    port: u16,
    request_timeout: Duration,

    url: &'a str,
    certificate: Option<&'a str>,
    max_connections: Option<u8>,
    allowed_updates: Option<&'a [UpdateKind]>,
}

impl<'a, C> Webhook<'a, C> {
    pub(crate) fn new(
        event_loop: EventLoop<C>,
        url: &'a str,
        port: u16,
    ) -> Self {
        Self {
            event_loop,
            ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
            request_timeout: Duration::from_secs(60),

            url,
            certificate: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    /// Configures the IP `tbot` will bind to.
    pub const fn ip(mut self, ip: IpAddr) -> Self {
        self.ip = ip;
        self
    }

    /// Configures `certificate`.
    pub fn certificate(mut self, certificate: &'a str) -> Self {
        self.certificate = Some(certificate);
        self
    }

    /// Configures `max_connections`.
    pub fn max_connections(mut self, max: u8) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Configures `allowed_updates`.
    pub fn allowed_updates(mut self, updates: &'a [UpdateKind]) -> Self {
        self.allowed_updates = Some(updates);
        self
    }

    /// Configures for how long `tbot` should wait for `setWebhook`. If you
    /// don't configure this value, it is set to `60s`.
    pub const fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Configures a webhook server over HTTP. For HTTPS, see the [`https`]
    /// method.
    ///
    /// [`https`]: #method.https
    pub const fn http(self) -> Http<'a, C> {
        Http::new(self)
    }

    /// Configures a webhook server over HTTPS. For HTTP, see the [`http`]
    /// method.
    ///
    /// [`http`]: #method.http
    pub const fn https(
        self,
        #[cfg(feature = "tls")] identity: https::Identity,
        #[cfg(feature = "rustls")] config: https::ServerConfig,
    ) -> Https<'a, C> {
        Https::new(
            self,
            #[cfg(feature = "tls")]
            identity,
            #[cfg(feature = "rustls")]
            config,
        )
    }
}

fn is_request_correct(request: &Request<Body>) -> bool {
    let content_type = request.headers().get("Content-Type");

    request.method() == Method::POST
        && request.uri() == "/"
        && content_type.map(|x| x == "application/json") == Some(true)
}

async fn handle<C>(
    bot: Arc<Bot<C>>,
    event_loop: Arc<EventLoop<C>>,
    request: Request<Body>,
) -> Result<Response<Body>, hyper::Error>
where
    C: Send + Sync + 'static,
{
    if is_request_correct(&request) {
        let (parts, mut body) = request.into_parts();
        let mut request = parts
            .headers
            .get("Content-Length")
            .and_then(|x| x.to_str().ok().and_then(|x| x.parse().ok()))
            .map_or_else(Vec::new, Vec::with_capacity);

        while let Some(chunk) = body.data().await {
            request.extend(chunk?);
        }

        let update =
            serde_json::from_slice(&request[..]).unwrap_or_else(|error| {
                panic!("\n[tbot] Received invalid JSON: {:#?}\n", error);
            });

        event_loop.handle_update(bot, update);
    }

    Ok(Response::new(Body::empty()))
}
