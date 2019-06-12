common! {
    /// The context for [`updated_poll`][handler] handlers.
    ///
    /// [handler]: ../struct.Bot.html#method.updated_poll
    struct UpdatedPoll {
        /// The new state of the poll.
        poll: types::Poll,
    }
}

impl UpdatedPoll {
    pub(crate) const fn new(bot: Arc<Bot>, poll: types::Poll) -> Self {
        Self {
            bot,
            poll,
        }
    }
}
