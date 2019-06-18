use std::sync::Arc;
use crate::Bot;

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
        update_id: u32,
    }
}


impl<C> Update<C> {
    pub(crate) const fn new(bot: Arc<Bot<C>>, update_id: u32) -> Self {
        Self {
            bot,
            update_id,
        }
    }
}
