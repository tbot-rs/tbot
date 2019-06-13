// use super::*;
use super::EventLoop;
use crate::{
    methods::{DeleteWebhook, GetUpdates},
    prelude::*,
    types,
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Configures and starts polling.
///
/// To construct `Polling`, use [`Bot::polling`].
///
/// [`Bot::polling`]: ./struct.Bot.html#method.polling
#[must_use = "polling does nothing unless `start` is called"]
pub struct Polling<'a, C> {
    event_loop: EventLoop<C>,
    limit: Option<u8>,
    timeout: Option<u32>,
    allowed_updates: Option<&'a [types::Updates]>,
    poll_interval: u64,
}

impl<'a, C> Polling<'a, C> {
    pub(crate) const fn new(event_loop: EventLoop<C>) -> Self {
        Self {
            event_loop,
            limit: None,
            timeout: None,
            allowed_updates: None,
            poll_interval: 25,
        }
    }

    /// Configures the limit of updates per request.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Configures the timeout for long polling.
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Configures which updates you'd like to listen to.
    pub fn allowed_updates(
        mut self,
        allowed_updates: &'a [types::Updates],
    ) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Configures the minimal interval between making requests.
    pub const fn poll_interval(mut self, poll_interval: u64) -> Self {
        self.poll_interval = poll_interval;
        self
    }
}

impl<'a, C> Polling<'a, C>
where
    C: hyper::client::connect::Connect + Clone + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    /// Starts the event loop.
    pub fn start(self) -> ! {
        self.delete_webhook();
        self.start_event_loop();
    }

    fn delete_webhook(&self) {
        let error = Arc::new(Mutex::new(None));
        let outer_error = Arc::clone(&error);

        let delete_webhook = DeleteWebhook::new(
            Arc::clone(&self.event_loop.bot.client),
            self.event_loop.bot.token.clone(),
        )
        .into_future()
        .map_err(move |error| *outer_error.lock().unwrap() = Some(error));

        crate::run(delete_webhook);

        let error = &*error.lock().unwrap();

        if let Some(error) = error {
            panic!(
                "\n[tbot] Error while deleting previous webhook: {:#?}\n",
                error,
            );
        }
    }

    fn start_event_loop(self) -> ! {
        let bot = Arc::new(self.event_loop.bot.clone());
        let event_loop = Arc::new(self.event_loop);
        let interval = Duration::from_millis(self.poll_interval);
        let last_offset = Arc::new(Mutex::new(None));
        let mut last_send_timestamp;

        loop {
            let bot = Arc::clone(&bot);
            let on_ok = Arc::clone(&event_loop);
            let on_error = Arc::clone(&event_loop);
            let new_offset = Arc::clone(&last_offset);

            last_send_timestamp = Instant::now();

            let updates = GetUpdates::new(
                Arc::clone(&event_loop.bot.client),
                event_loop.bot.token.clone(),
                *last_offset.lock().unwrap(),
                self.limit,
                self.timeout,
                self.allowed_updates,
            )
            .into_future();

            let handler = updates
                .map(move |updates| {
                    if let Some(update) = updates.last() {
                        *new_offset.lock().unwrap() = Some(update.id + 1);
                    }

                    for update in updates {
                        on_ok.handle_update(Arc::clone(&bot), update);
                    }
                })
                .map_err(move |error| {
                    on_error.run_polling_error_handlers(&error)
                });

            crate::run(handler);

            let next_timestamp = last_send_timestamp + interval;
            let now = Instant::now();

            if next_timestamp > now {
                std::thread::sleep(next_timestamp - now);
            }
        }
    }
}
