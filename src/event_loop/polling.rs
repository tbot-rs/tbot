// use super::*;
use super::EventLoop;
use crate::{
    errors,
    internal::BoxFuture,
    methods::{DeleteWebhook, GetUpdates},
    prelude::*,
    types::parameters::Updates,
};
use futures::Stream;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    timer::{self, timeout},
    util::FutureExt,
};

mod schedule;

use schedule::Schedule;

type Error = timeout::Error<errors::MethodCall>;
type ErrorHandler = dyn FnMut(Error) + Send + Sync;

/// Configures and starts polling.
///
/// To construct `Polling`, use [`Bot::polling`].
///
/// [`Bot::polling`]: ./struct.Bot.html#method.polling
#[must_use = "polling does nothing unless `start` is called"]
pub struct Polling<C> {
    event_loop: EventLoop<C>,
    limit: Option<u8>,
    timeout: Option<u64>,
    allowed_updates: Option<&'static [Updates]>,
    poll_interval: Duration,
    error_handler: Mutex<Box<ErrorHandler>>,
    request_timeout: Option<Duration>,
}

impl<C> Polling<C> {
    pub(crate) fn new(event_loop: EventLoop<C>) -> Self {
        Self {
            event_loop,
            limit: None,
            timeout: None,
            allowed_updates: None,
            poll_interval: Duration::from_millis(25),
            error_handler: Mutex::new(Box::new(|err| {
                panic!("\n[tbot] Polling error: {:#?}", err);
            })),
            request_timeout: None,
        }
    }

    /// Configures the limit of updates per request.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Configures the timeout for long polling.
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Configures which updates you'd like to listen to.
    pub fn allowed_updates(
        mut self,
        allowed_updates: &'static [Updates],
    ) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a handler for errors ocurred while polling.
    pub fn error_handler(
        mut self,
        handler: impl FnMut(Error) + Send + Sync + 'static,
    ) -> Self {
        self.error_handler = Mutex::new(Box::new(handler));
        self
    }

    /// Configures the minimal interval between making requests. Set to `25ms`
    /// by default.
    pub const fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Configures for how long `tbot` should wait for `getUpdates`. If this
    /// timeout is exceeded, an [error handler] is triggered. If you don't
    /// configure this value, it is set to
    /// `Duration::from_secs(timeout.unwrap_or(0) + 60)`.
    ///
    /// [error handler]: #method.error_handler
    pub const fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
}

impl<C> Polling<C>
where
    C: hyper::client::connect::Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    /// Starts the event loop.
    pub fn start(self) -> ! {
        let future = self
            .delete_webhook()
            .map_err(|err| {
                eprintln!(
                    "[tbot] Error while deleting previous webhook. \
                     Starting polling anyway. {:#?}",
                    err,
                );
            })
            .then(|_| {
                self.into_future().map_err(|err| {
                    eprintln!("[tbot] Poll schedule error: {:#?}", err);
                })
            });

        crate::run(future);

        panic!(
            "\n[tbot] Polling event loop unexpected returned. \
             An error should be printed above.\n"
        );
    }

    /// Deleted the webhook of this bot.
    pub fn delete_webhook(&self) -> impl Future<Item = (), Error = Error> {
        let request_timeout = self.request_timeout.unwrap_or_else(|| {
            Duration::from_secs(self.timeout.unwrap_or(0) + 60)
        });

        DeleteWebhook::new(
            &self.event_loop.bot.client,
            self.event_loop.bot.token.clone(),
        )
        .into_future()
        .timeout(request_timeout)
    }
}

impl<C> IntoFuture for Polling<C>
where
    C: hyper::client::connect::Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = timer::Error;

    fn into_future(self) -> Self::Future {
        let Self {
            event_loop,
            poll_interval,
            limit,
            timeout,
            allowed_updates,
            error_handler,
            request_timeout,
        } = self;

        let request_timeout = request_timeout
            .unwrap_or_else(|| Duration::from_secs(timeout.unwrap_or(0) + 60));

        let bot = Arc::new(event_loop.bot.clone());
        let event_loop = Arc::new(event_loop);
        let error_handler = Arc::new(error_handler);

        let stream = Schedule::new(poll_interval).into_stream();

        let stream = stream.for_each(move |(last_offset, schedule)| {
            let bot = Arc::clone(&bot);
            let event_loop = Arc::clone(&event_loop);
            let error_handler = Arc::clone(&error_handler);
            let on_error_schedule = Arc::clone(&schedule);

            let handler = GetUpdates::new(
                &event_loop.bot.client,
                event_loop.bot.token.clone(),
                last_offset,
                limit,
                timeout,
                allowed_updates,
            )
            .into_future()
            .map(move |updates| {
                let mut schedule = schedule.lock().unwrap();

                if let Some(update) = updates.last() {
                    schedule.last_offset = Some(update.id.0 + 1);
                }

                schedule.schedule_next_tick();
                std::mem::drop(schedule);

                for update in updates {
                    event_loop.handle_update(Arc::clone(&bot), update);
                }
            })
            .timeout(request_timeout)
            .map_err(move |error| {
                (&mut *(*error_handler).lock().unwrap())(error);

                on_error_schedule.lock().unwrap().schedule_next_tick();
            });

            crate::spawn(handler);

            Ok(())
        });

        Box::new(stream)
    }
}
