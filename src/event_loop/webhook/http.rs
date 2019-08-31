use crate::{
    errors,
    event_loop::{EventLoop, Webhook},
    internal::BoxFuture,
    Bot,
};
use futures::{future::Either, Future, IntoFuture, Stream};
use hyper::{
    client::connect::Connect, service::service_fn, Body, Method, Request,
    Response, Server,
};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::util::FutureExt;

pub struct Http<'a, C> {
    webhook: Webhook<'a, C>,
}

impl<'a, C> Http<'a, C> {
    pub(crate) fn new(webhook: Webhook<'a, C>) -> Self {
        Self { webhook }
    }
}

impl<'a, C> Http<'a, C>
where
    C: Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    /// Starts the server.
    pub fn start(self) -> ! {
        crate::run(self.into_future().map_err(|err| {
            eprintln!("\n[tbot] Webhook error: {:#?}", err);
        }));

        unreachable!(
            "\n[tbot] The webhook server unexpectedly returned. \
             An error should be printed above.\n",
        );
    }
}

impl<'a, C> IntoFuture for Http<'a, C>
where
    C: Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::HttpWebhook;

    fn into_future(self) -> Self::Future {
        let set_webhook = self
            .webhook
            .event_loop
            .bot
            .set_webhook(
                self.webhook.url,
                self.webhook.certificate,
                self.webhook.max_connections,
                self.webhook.allowed_updates,
            )
            .into_future()
            .timeout(self.webhook.request_timeout)
            .map_err(errors::HttpWebhook::SetWebhook);

        let Webhook {
            event_loop,
            ip,
            port,
            ..
        } = self.webhook;

        Box::new(set_webhook.and_then(move |_| {
            init_server(event_loop, ip, port)
                .map_err(errors::HttpWebhook::Server)
        }))
    }
}

fn init_server<C>(
    event_loop: EventLoop<C>,
    ip: IpAddr,
    port: u16,
) -> impl Future<Item = (), Error = hyper::Error>
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
) -> impl Future<Item = Response<Body>, Error = hyper::Error>
where
    C: Send + Sync + 'static,
{
    if is_request_correct(&request) {
        let body = request.into_body().concat2();
        let handler = body.map(move |body| {
            match serde_json::from_slice(&body[..]) {
                Ok(update) => event_loop.handle_update(bot, update),
                Err(error) => {
                    eprintln!(
                        "[tbot] Could not parse incoming update:\n\n\
                         Request (in bytes): {request:?}\n\
                         Error: {error:#?}",
                        request = &body[..],
                        error = error
                    );
                }
            }

            Response::new(Body::empty())
        });

        Either::A(handler)
    } else {
        let response = Response::new(Body::empty());
        let future = futures::future::ok(response);

        Either::B(future)
    }
}
