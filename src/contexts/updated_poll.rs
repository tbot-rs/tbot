common! {
    /// Context for the [`updated_poll`] handler.
    ///
    /// [`updated_poll`]: ../struct.Bot.html#method.updated_poll
    struct UpdatedPollContext {
        /// The unhandled update.
        poll: types::Poll,
    }
}

impl UpdatedPollContext {
    pub(crate) const fn new(bot: Arc<MockBot>, poll: types::Poll) -> Self {
        Self {
            bot,
            poll,
        }
    }
}
