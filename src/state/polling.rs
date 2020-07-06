use crate::{
    errors,
    event_loop::{self, EventLoop},
    types::parameters::UpdateKind,
};
use std::{convert::Infallible, num::NonZeroUsize, sync::Arc, time::Duration};

/// Configures and starts polling for the stateful event loop.
///
/// To construct `Polling`, use [`StatefulEventLoop::polling`].
///
/// [`StatefulEventLoop::polling`]: ./struct.StatefulEventLoop.html#method.polling
#[must_use = "polling does nothing unless `start` is called"]
pub struct Polling<S> {
    inner: event_loop::Polling,
    state: Arc<S>,
}

#[allow(clippy::use_self)] // https://github.com/rust-lang/rust-clippy/issues/4143
impl<S> Polling<S> {
    pub(crate) fn new(event_loop: EventLoop, state: Arc<S>) -> Self {
        Self {
            inner: event_loop::Polling::new(event_loop),
            state,
        }
    }

    /// Turns this polling into a stateless one. Previous configuration
    // is preserved.
    #[allow(clippy::missing_const_for_fn)] // https://github.com/rust-lang/rust-clippy/issues/4979
    pub fn into_stateless(self) -> event_loop::Polling {
        self.inner
    }

    /// Turns this polling into another with other state.
    pub fn with_other_state<T>(self, other_state: T) -> Polling<T>
    where
        T: Send + Sync + 'static,
    {
        Polling {
            inner: self.inner,
            state: Arc::new(other_state),
        }
    }

    /// Configures the limit of updates per request.
    pub fn limit(mut self, limit: u8) -> Self {
        self.inner = self.inner.limit(limit);
        self
    }

    /// Configures the timeout for long polling.
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.inner = self.inner.timeout(timeout);
        self
    }

    /// Configures which updates you'd like to listen to.
    pub fn allowed_updates(
        mut self,
        allowed_updates: &'static [UpdateKind],
    ) -> Self {
        self.inner = self.inner.allowed_updates(allowed_updates);
        self
    }

    /// Configures the minimal interval between making requests. Set to `25ms`
    /// by default.
    #[allow(clippy::missing_const_for_fn)] // https://github.com/rust-lang/rust-clippy/issues/4979
    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.inner = self.inner.poll_interval(poll_interval);
        self
    }

    /// Configures for how long `tbot` should wait for `getUpdates`. If this
    /// timeout is exceeded, an [error handler] is triggered. If you don't
    /// configure this value, it is set to
    /// `Duration::from_secs(timeout.unwrap_or(0) + 60)`.
    ///
    /// [error handler]: #method.error_handler
    #[allow(clippy::missing_const_for_fn)] // https://github.com/rust-lang/rust-clippy/issues/4979
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.inner = self.inner.request_timeout(timeout);
        self
    }

    /// Configures how many updates `tbot` will process on start. If configured,
    /// `tbot` sets `offset`'s value to `-n` when making the first request.
    ///
    /// # Panics
    ///
    /// Panics if `n` can't be converted to `isize` safely.
    pub fn last_n_updates(mut self, n: NonZeroUsize) -> Self {
        self.inner = self.inner.last_n_updates(n);
        self
    }

    /// Starts the event loop.
    #[allow(clippy::future_not_send)] // `S: Send + Sync` is guaranteed
    pub async fn start(self) -> Result<Infallible, errors::PollingSetup> {
        self.inner.start().await
    }
}

impl<S> Polling<S>
where
    S: Send + Sync + 'static,
{
    /// Adds a handler for errors ocurred while polling.
    pub fn error_handler<H, F>(mut self, handler: H) -> Self
    where
        H: (Fn(errors::Polling, Arc<S>) -> F) + Send + Sync + 'static,
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);

        self.inner = self.inner.error_handler(move |error| {
            let state = Arc::clone(&state);

            handler(error, state)
        });
        self
    }
}
