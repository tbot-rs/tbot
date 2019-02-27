use super::*;
use futures::Stream;
use hyper::{
    service::service_fn, Body, Error, Method, Request, Response, Server,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// Configures webhook and starts a server.
///
/// To construct a `Webhook`, use [`Bot::webhook`].
///
/// [`Bot::webhook`]: ./struct.Bot.html#method.webhook
#[must_use = "webhook does not start unless `start` is called"]
pub struct Webhook<'a> {
    bot: Bot,
    ip: IpAddr,
    port: u16,

    url: &'a str,
    certificate: Option<&'a str>,
    max_connections: Option<u8>,
    allowed_updates: Option<&'a [types::Updates]>,
}

impl<'a> Webhook<'a> {
    pub(crate) fn new(bot: Bot, url: &'a str, port: u16) -> Self {
        Self {
            bot,
            ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
            url,
            certificate: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    /// Configures the IP `tbot` will bind to.
    pub fn ip(mut self, ip: IpAddr) -> Self {
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
    pub fn allowed_updates(mut self, updates: &'a [types::Updates]) -> Self {
        self.allowed_updates = Some(updates);
        self
    }

    /// Starts the server.
    pub fn start(self) -> ! {
        self.set_webhook();
        self.start_event_loop();
    }

    fn set_webhook(&self) {
        let set_webhook = methods::SetWebhook::new(
            &self.bot.token,
            self.url,
            self.certificate,
            self.max_connections,
            self.allowed_updates,
            #[cfg(feature = "proxy")]
            self.bot.proxy.clone(),
        )
        .into_future();

        if let Err(error) = set_webhook.wait() {
            panic!("\n[tbot] Error while setting webhook: {:#?}\n", error);
        }
    }

    fn start_event_loop(self) -> ! {
        let server = init_server(self.bot, self.ip, self.port);

        if let Err(error) = server.wait() {
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

fn handle(
    bot: Arc<Bot>,
    request: Request<Body>,
) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send> {
    if is_request_correct(&request) {
        let body = request.into_body().concat2();
        let handler = body.map(move |body| {
            let update =
                serde_json::from_slice(&body[..]).unwrap_or_else(|error| {
                    panic!("\n[tbot] Received invalid JSON: {:#?}\n", error);
                });

            bot.handle_update(update);

            Response::new(Body::empty())
        });

        Box::new(handler)
    } else {
        let response = Response::new(Body::empty());
        let future = futures::future::ok(response);

        Box::new(future)
    }
}

fn init_server(bot: Bot, ip: IpAddr, port: u16) -> impl Future<Error = Error> {
    let bot = Arc::new(bot);
    let addr = SocketAddr::new(ip, port);

    Server::bind(&addr).serve(move || {
        let bot = bot.clone();
        service_fn(move |request| handle(bot.clone(), request))
    })
}
