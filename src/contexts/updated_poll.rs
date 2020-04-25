use crate::{types::Poll, Bot};
use std::sync::Arc;

common! {
    /// The context for [`updated_poll`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.updated_poll
    struct UpdatedPoll {
        /// The new state of the poll.
        poll: Poll,
    }
}

impl UpdatedPoll {
    pub(crate) const fn new(bot: Arc<Bot>, poll: Poll) -> Self {
        Self { bot, poll }
    }
}
