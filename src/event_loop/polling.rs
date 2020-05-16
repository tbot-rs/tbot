use super::EventLoop;
use crate::{errors, types::parameters::UpdateKind};
use std::{
    convert::{Infallible, TryInto},
    num::NonZeroUsize,
    sync::Arc,
    time::Duration,
};
use tokio::time::{delay_for, timeout as timeout_future};
use tracing::instrument;

type ErrorHandler = dyn Fn(errors::Polling) + Send + Sync;

/// Configures and starts polling.
///
/// To construct `Polling`, use [`Bot::polling`].
///
/// [`Bot::polling`]: ./struct.Bot.html#method.polling
#[must_use = "polling does nothing unless `start` is called"]
pub struct Polling {
    event_loop: EventLoop,
    limit: Option<u8>,
    timeout: Option<u64>,
    allowed_updates: Option<&'static [UpdateKind]>,
    poll_interval: Duration,
    error_handler: Box<ErrorHandler>,
    request_timeout: Option<Duration>,
    offset: Option<isize>,
}

impl Polling {
    pub(crate) fn new(event_loop: EventLoop) -> Self {
        Self {
            event_loop,
            limit: None,
            timeout: None,
            allowed_updates: None,
            poll_interval: Duration::from_millis(25),
            error_handler: Box::new(|err| {
                eprintln!("[tbot] Polling error: {:#?}", err);
            }),
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
        allowed_updates: &'static [UpdateKind],
    ) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a handler for errors ocurred while polling.
    pub fn error_handler<H, F>(mut self, handler: H) -> Self
    where
        H: (Fn(errors::Polling) -> F) + Send + Sync + 'static,
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.error_handler = Box::new(move |error| {
            tokio::spawn(handler(error));
        });
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

impl Polling {
    /// Starts the event loop.
    #[instrument(name = "polling", skip(self))]
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

        let set_commands = event_loop.set_commands_descriptions();
        timeout_future(request_timeout, set_commands).await?;

        let bot = Arc::new(event_loop.bot.clone());

        loop {
            let mut next_tick = delay_for(poll_interval);

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
                Ok(Err(error)) => {
                    if let errors::MethodCall::RequestError {
                        retry_after: Some(retry_after),
                        ..
                    } = error
                    {
                        next_tick = delay_for(Duration::from_secs(retry_after));
                    }

                    error_handler(error.into());
                }
                Err(error) => error_handler(error.into()),
            }

            next_tick.await
        }
    }
}
