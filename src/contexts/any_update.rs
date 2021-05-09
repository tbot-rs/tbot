use crate::{
    types::update::{self, Update},
    Bot,
};

common! {
    /// The context for [`any_update`] handlers.
    ///
    /// [`any_update`]: crate::EventLoop::any_update
    struct AnyUpdate {
        /// The update's ID.
        id: update::Id,
        /// The update's kind.
        kind: update::Kind,
    }
}

impl AnyUpdate {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Bot, update: Update) -> Self {
        Self {
            bot,
            id: update.id,
            kind: update.kind,
        }
    }
}
