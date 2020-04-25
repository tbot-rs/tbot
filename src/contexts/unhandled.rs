use crate::{types::update, Bot};
use std::sync::Arc;

common! {
    /// The context for [`unhandled`] handlers.
    ///
    /// [`unhandled`]: ../event_loop/struct.EventLoop.html#method.unhandled
    struct Unhandled {
        /// The unhandled update.
        update: update::Kind,
    }
}

impl Unhandled {
    pub(crate) const fn new(bot: Arc<Bot>, update: update::Kind) -> Self {
        Self { bot, update }
    }
}
