use super::EventLoop;
use crate::{
    errors, internal::BoxFuture, prelude::*, types::parameters::Updates, Bot,
};
use futures::{future::Either, Stream};
use hyper::{
    client::connect::Connect, service::service_fn, Body, Method, Request,
    Response, Server,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::util::FutureExt;

mod http;

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
    allowed_updates: Option<&'a [Updates]>,
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
    pub fn allowed_updates(mut self, updates: &'a [Updates]) -> Self {
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
    pub fn http(self) -> http::Http<'a, C> {
        http::Http::new(self)
    }
}
