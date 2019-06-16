// use super::*;
use super::EventLoop;
use crate::{methods, prelude::*, types::parameters::Updates, Bot};
use futures::Stream;
use hyper::{
    service::service_fn, Body, Error, Method, Request, Response, Server,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
};

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
}

impl<'a, C> Webhook<'a, C>
where
    C: hyper::client::connect::Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    /// Starts the server.
    pub fn start(self) -> ! {
        self.set_webhook();
        self.start_event_loop();
    }

    fn set_webhook(&self) {
        let error = Arc::new(Mutex::new(None));
        let outer_error = Arc::clone(&error);

        let set_webhook = methods::SetWebhook::new(
            &self.event_loop.bot.client,
            self.event_loop.bot.token.clone(),
            self.url,
            self.certificate,
            self.max_connections,
            self.allowed_updates,
        )
        .into_future()
        .map_err(move |error| *outer_error.lock().unwrap() = Some(error));

        crate::run(set_webhook);

        let error = &*error.lock().unwrap();

        if let Some(error) = error {
            panic!("\n[tbot] Error while setting webhook: {:#?}\n", error);
        }
    }

    fn start_event_loop(self) -> ! {
        let error = Arc::new(Mutex::new(None));
        let outer_error = Arc::clone(&error);
        let server = init_server(self.event_loop, self.ip, self.port)
            .map_err(move |error| *outer_error.lock().unwrap() = Some(error));

        crate::run(server);

        let error = &*error.lock().unwrap();

        if let Some(error) = error {
            panic!(
                "\n[tbot] Webhook server returned with an error: {:#?}\n",
                error,
            );
        }

        unreachable!();
    }
}

fn is_request_correct(request: &Request<Body>) -> bool {
    let content_type = request.headers().get("Content-Type");

    request.method() == Method::POST
        && request.uri() == "/"
        && content_type.map(|x| x == "application/json") == Some(true)
}

fn handle<C>(
    bot: Arc<Bot<C>>,
    event_loop: Arc<EventLoop<C>>,
    request: Request<Body>,
) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send>
where
    C: Send + Sync + 'static,
{
    if is_request_correct(&request) {
        let body = request.into_body().concat2();
        let handler = body.map(move |body| {
            let update =
                serde_json::from_slice(&body[..]).unwrap_or_else(|error| {
                    panic!("\n[tbot] Received invalid JSON: {:#?}\n", error);
                });

            event_loop.handle_update(bot, update);

            Response::new(Body::empty())
        });

        Box::new(handler)
    } else {
        let response = Response::new(Body::empty());
        let future = futures::future::ok(response);

        Box::new(future)
    }
}

fn init_server<C>(
    event_loop: EventLoop<C>,
    ip: IpAddr,
    port: u16,
) -> impl Future<Error = Error>
where
    C: Clone + Send + Sync + 'static,
{
    let bot = Arc::new(event_loop.bot.clone());
    let event_loop = Arc::new(event_loop);
    let addr = SocketAddr::new(ip, port);

    Server::bind(&addr).serve(move || {
        let bot = Arc::clone(&bot);
        let event_loop = Arc::clone(&event_loop);
        service_fn(move |request| {
            handle(Arc::clone(&bot), Arc::clone(&event_loop), request)
        })
    })
}
