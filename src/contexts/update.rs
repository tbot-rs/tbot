use super::*;
use std::sync::Arc;

/// The context for [`before_update`] and [`after_update`] handlers.
///
/// This context does _not_ provide the contents of the update. Use dedicated
/// update handlers instead.
///
/// [`before_update`]: ../struct.Bot.html#method.before_update
/// [`after_update`]: ../struct.Bot.html#method.after_update
#[derive(Clone)]
// todo: #[non_exhaustive]
pub struct Update {
    /// A mock bot with all API methods.
    pub bot: Arc<Bot>,
    /// The ID of the update.
    pub update_id: u32,
}

impl Update {
    pub(crate) const fn new(bot: Arc<Bot>, update_id: u32) -> Self {
        Self {
            bot,
            update_id,
        }
    }
}
