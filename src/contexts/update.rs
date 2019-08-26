use crate::{types::update, Bot};
use std::sync::Arc;

common! {
    /// The context for [`before_update`][before] and [`after_update`][after]
    /// handlers.
    ///
    /// This context does _not_ provide the contents of the update. Use dedicated
    /// update handlers instead.
    ///
    /// [before]: ../event_loop/struct.EventLoop.html#method.before_update
    /// [after]: ../event_loop/struct.EventLoop.html#method.after_update
    struct Update {
        /// The ID of the update.
        update_id: update::Id,
    }
}

impl<C> Update<C> {
    pub(crate) const fn new(bot: Arc<Bot<C>>, update_id: update::Id) -> Self {
        Self { bot, update_id }
    }
}
