common! {
    /// The context for [`updated_poll`][handler] handlers.
    ///
    /// [handler]: ../struct.Bot.html#method.updated_poll
    struct UpdatedPoll {
        /// The new state of the poll.
        poll: types::Poll,
    }
}

impl<C> UpdatedPoll<C> {
    pub(crate) const fn new(bot: Arc<Bot<C>>, poll: types::Poll) -> Self {
        Self {
            bot,
            poll,
        }
    }
}
