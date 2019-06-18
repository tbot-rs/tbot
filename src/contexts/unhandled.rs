use std::sync::Arc;
use crate::{Bot, types::update};

common! {
    /// The context for [`unhandled`] handlers.
    ///
    /// [`unhandled`]: ../event_loop/struct.EventLoop.html#method.unhandled
    struct Unhandled {
        /// The unhandled update.
        update: update::Kind,
    }
}

impl<C> Unhandled<C> {
    pub(crate) const fn new(
        bot: Arc<Bot<C>>,
        update: update::Kind,
    ) -> Self {
        Self {
            bot,
            update,
        }
    }
}
