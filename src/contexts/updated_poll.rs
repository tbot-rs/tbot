common! {
    /// Context for the [`updated_poll`] handler.
    ///
    /// [`updated_poll`]: ../struct.Bot.html#method.updated_poll
    struct UpdatedPoll {
        /// The unhandled update.
        poll: types::Poll,
    }
}

impl UpdatedPoll {
    pub(crate) const fn new(bot: Arc<MockBot>, poll: types::Poll) -> Self {
        Self {
            bot,
            poll,
        }
    }
}
