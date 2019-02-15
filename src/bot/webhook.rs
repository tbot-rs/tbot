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
    pub fn max_connections(mut self, amount: u8) -> Self {
        self.max_connections = Some(amount);
        self
    }

    /// Configures `allowed_updates`.
    pub fn allowed_updates(mut self, updates: &'a [types::Updates]) -> Self {
        self.allowed_updates = Some(updates);
        self
    }

    /// Starts the server.
    pub fn start(self) -> ! {
        let error = Arc::new(Mutex::new(Ok(())));
        let handler = error.clone();
        let webhook_set_request = methods::SetWebhook::new(
            &self.bot.token,
            self.url,
            self.certificate,
            self.max_connections,
            self.allowed_updates,
            #[cfg(feature = "proxy")]
            self.bot.proxy.clone(),
        )
        .into_future()
        .map_err(move |error| *handler.lock().unwrap() = Err(error));

        crate::run(webhook_set_request);

        if let Err(error) = &*error.lock().unwrap() {
            panic!("\n[tbot] error while setting webhook:\n\n{:#?}\n", error);
        }

        start_server(Arc::new(self.bot), self.ip, self.port);
    }
}

fn is_content_type_correct(request: &Request<Body>) -> bool {
    let header = request.headers().get("Content-Type");

    header.map(|x| x == "application/json") == Some(true)
}

fn handle(
    bot: Arc<Bot>,
    request: Request<Body>,
) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send> {
    if request.method() == Method::POST && is_content_type_correct(&request) {
        Box::new(request.into_body().concat2().map(move |body| {
            let update = serde_json::from_slice(&body[..]).unwrap();
            bot.handle_update(update);
            Response::new(Body::empty())
        }))
    } else {
        Box::new(futures::future::ok(Response::new(Body::empty())))
    }
}

fn start_server(bot: Arc<Bot>, ip: IpAddr, port: u16) -> ! {
    let addr = SocketAddr::new(ip, port);

    let server = Server::bind(&addr)
        .serve(move || {
            let bot = bot.clone();
            service_fn(move |request| handle(bot.clone(), request))
        })
        .map_err(|error| {
            eprintln!("\n[tbot] webhook server error:\n\n{:#?}\n", error);
        });

    tokio::run(server);
    unreachable!(
        "\n[tbot] webhook server was expected to never return. Perhaps there's \
        an error logged above\n",
    );
}
