use super::*;
use std::sync::Arc;

/// Context for the [`unhandled`] handler.
///
/// [`unhandled`]: ../struct.Bot.html#method.unhandled
#[derive(Clone)]
pub struct UnhandledContext {
    /// A mock bot with all API methods.
    pub bot: Arc<MockBot>,
    /// The unhandled update.
    pub update: types::UpdateKind,
}

impl UnhandledContext {
    pub(crate) const fn new(
        bot: Arc<MockBot>,
        update: types::UpdateKind,
    ) -> Self {
        Self {
            bot,
            update,
        }
    }
}
