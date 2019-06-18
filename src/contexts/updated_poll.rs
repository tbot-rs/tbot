use std::sync::Arc;
use crate::{Bot, types::Poll};

common! {
    /// The context for [`updated_poll`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.updated_poll
    struct UpdatedPoll {
        /// The new state of the poll.
        poll: Poll,
    }
}

impl<C> UpdatedPoll<C> {
    pub(crate) const fn new(bot: Arc<Bot<C>>, poll: Poll) -> Self {
        Self {
            bot,
            poll,
        }
    }
}
