use super::*;
use std::sync::Arc;

/// Context for the [`updated_poll`] handler.
///
/// [`updated_poll`]: ../struct.Bot.html#method.updated_poll
#[derive(Clone)]
pub struct UpdatedPollContext {
    /// A mock bot with all API methods.
    pub bot: Arc<MockBot>,
    /// The poll.
    pub poll: types::Poll,
}

impl UpdatedPollContext {
    pub(crate) const fn new(bot: Arc<MockBot>, poll: types::Poll) -> Self {
        Self {
            bot,
            poll,
        }
    }
}
