use crate::{types::poll::Answer, Bot};

common! {
    /// The context for [`poll_answer`] handlers.
    ///
    /// [`poll_answer`]: crate::EventLoop::poll_answer
    struct PollAnswer {
        /// The new answer in the poll.
        answer: Answer,
    }
}

impl PollAnswer {
    pub(crate) const fn new(bot: Bot, answer: Answer) -> Self {
        Self { bot, answer }
    }
}
