use crate::{types::poll::Answer, Bot};
use std::sync::Arc;

common! {
    /// The context for [`poll_answer`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.poll_answer
    struct PollAnswer {
        /// The new answer in the poll.
        answer: Answer,
    }
}

impl PollAnswer {
    pub(crate) const fn new(bot: Arc<Bot>, answer: Answer) -> Self {
        Self { bot, answer }
    }
}
