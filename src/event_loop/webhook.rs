//! Types related to the webhook event loop.

use super::EventLoop;
use crate::types::parameters::AllowedUpdates;
use hyper::{
    body::{Body, HttpBody},
    Method, Request, Response,
};
use std::{
    net::{IpAddr, Ipv4Addr},
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

mod http;
pub mod https;

pub use http::Http;
pub use https::Https;

/// Configures webhook and starts a server.
///
/// To construct a `Webhook`, use [`EventLoop::webhook`].
///
/// [`EventLoop::webhook`]: ./struct.Bot.html#method.webhook
#[must_use = "webhook does not start unless `start` is called"]
pub struct Webhook<'a> {
    event_loop: EventLoop,
    bind_to: IpAddr,
    port: u16,
    request_timeout: Duration,
    updates_url: String,

    url: &'a str,
    ip_address: Option<IpAddr>,
    certificate: Option<&'a str>,
    max_connections: Option<NonZeroU32>,
    allowed_updates: Option<AllowedUpdates>,
    drop_pending_updates: bool,
}

impl<'a> Webhook<'a> {
    pub(crate) fn new(event_loop: EventLoop, url: &'a str, port: u16) -> Self {
        Self {
            event_loop,
            bind_to: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
            request_timeout: Duration::from_secs(60),
            updates_url: String::from("/"),

            url,
            ip_address: None,
            certificate: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: false,
        }
    }

    /// Configures the IP `tbot` will bind to.
    pub const fn bind_to(mut self, bind_to: IpAddr) -> Self {
        self.bind_to = bind_to;
        self
    }

    /// Configures the URL that `tbot` will accept connections to.
    /// `/` by default.
    ///
    /// # Panics
    ///
    /// Panics if the URL doesn't starts with `/`.
    pub fn accept_updates_on(mut self, url: String) -> Self {
        if !url.starts_with('/') {
            panic!(
                "[tbot] `Webhook::accept_connections_to` takes URLs starting \
                only with `/`"
            );
        }

        self.updates_url = url;
        self
    }

    /// Configures the IP address which the Bot API server will use to send
    /// updates avoiding DNS. Reflects the `ip_address` parameter.
    pub fn ip_address(mut self, ip_address: impl Into<IpAddr>) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// Configures `certificate`.
    pub const fn certificate(mut self, certificate: &'a str) -> Self {
        self.certificate = Some(certificate);
        self
    }

    /// Configures `max_connections`.
    pub const fn max_connections(mut self, max: NonZeroU32) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Configures `allowed_updates`.
    pub const fn allowed_updates(mut self, updates: AllowedUpdates) -> Self {
        self.allowed_updates = Some(updates);
        self
    }

    /// Sets `drop_pending_updates` to `true`.
    pub const fn drop_pending_updates(mut self) -> Self {
        self.drop_pending_updates = true;
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
    pub const fn http(self) -> Http<'a> {
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
    ) -> Https<'a> {
        Https::new(
            self,
            #[cfg(feature = "tls")]
            identity,
            #[cfg(feature = "rustls")]
            config,
        )
    }
}

fn is_request_correct(request: &Request<Body>, updates_url: &str) -> bool {
    let content_type = request.headers().get("Content-Type");

    request.method() == Method::POST
        && request.uri() == updates_url
        && content_type.map(|x| x == "application/json") == Some(true)
}

async fn handle(
    event_loop: Arc<EventLoop>,
    request: Request<Body>,
    updates_url: Arc<String>,
) -> Result<Response<Body>, hyper::Error> {
    if is_request_correct(&request, &*updates_url) {
        let (parts, mut body) = request.into_parts();
        let mut request = parts
            .headers
            .get("Content-Length")
            .and_then(|x| x.to_str().ok().and_then(|x| x.parse().ok()))
            .map_or_else(Vec::new, Vec::with_capacity);

        while let Some(chunk) = body.data().await {
            request.extend(chunk?);
        }

        match serde_json::from_slice(&request[..]) {
            Ok(update) => event_loop.handle_update(update),
            Err(error) => eprintln!(
                "[tbot] Failed to parse an update: {:?}. `tbot` will skip it, \
                 but this error means that `tbot`'s type deserialization \
                 doesn't match the Bot API. You should file an issue at \
                 https://gitlab.com/SnejUgal/tbot.",
                error
            ),
        }
    }

    Ok(Response::new(Body::empty()))
}
