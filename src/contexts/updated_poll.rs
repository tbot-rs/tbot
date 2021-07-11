use crate::{types::Poll, Bot};

common! {
    /// The context for [`updated_poll`] handlers.
    ///
    /// [`updated_poll`]: crate::EventLoop::updated_poll
    struct UpdatedPoll {
        /// The new state of the poll.
        poll: Poll,
    }
}

impl UpdatedPoll {
    pub(crate) const fn new(bot: Bot, poll: Poll) -> Self {
        Self { bot, poll }
    }
}
