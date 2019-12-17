use super::EventLoop;
use crate::{connectors::Connector, errors, types::parameters::Updates};
use std::{
    convert::{Infallible, TryInto},
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::{delay_for, timeout as timeout_future};

type ErrorHandler = dyn FnMut(errors::Polling) + Send + Sync;

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
    offset: Option<isize>,
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
                eprintln!("[tbot] Polling error: {:#?}", err);
            })),
            request_timeout: None,
            offset: None,
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
        handler: impl FnMut(errors::Polling) + Send + Sync + 'static,
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

    /// Configures how many updates `tbot` will process on start. If configured,
    /// `tbot` sets `offset`'s value to `-n` when making the first request.
    ///
    /// # Panics
    ///
    /// Panics if `n` can't be converted to `isize` safely.
    pub fn last_n_updates(mut self, n: NonZeroUsize) -> Self {
        let n: isize = n.get().try_into().unwrap_or_else(|_| {
            panic!("\n[tbot] Cannot convert {} to isize safely\n", n);
        });
        self.offset = Some(-n);
        self
    }
}

impl<C: Connector + Clone> Polling<C> {
    /// Starts the event loop.
    pub async fn start(self) -> Result<Infallible, errors::PollingSetup> {
        let Self {
            event_loop,
            poll_interval,
            limit,
            timeout,
            allowed_updates,
            error_handler,
            request_timeout,
            mut offset,
        } = self;

        let request_timeout = request_timeout
            .unwrap_or_else(|| Duration::from_secs(timeout.unwrap_or(0) + 60));

        let delete_webhook = event_loop.bot.delete_webhook().call();
        timeout_future(request_timeout, delete_webhook).await??;

        let bot = Arc::new(event_loop.bot.clone());
        let error_handler = &mut *error_handler.lock().unwrap();

        loop {
            let next_tick = delay_for(poll_interval);

            let get_updates = bot
                .get_updates(offset, limit, timeout, allowed_updates)
                .call();

            match timeout_future(request_timeout, get_updates).await {
                Ok(Ok(updates)) => {
                    if let Some(update) = updates.last() {
                        offset = Some(update.id.0 + 1);
                    }

                    for update in updates {
                        event_loop.handle_update(Arc::clone(&bot), update);
                    }
                }
                Ok(Err(error)) => error_handler(error.into()),
                Err(error) => error_handler(error.into()),
            }

            next_tick.await
        }
    }
}
