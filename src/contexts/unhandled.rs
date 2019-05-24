common! {
    /// Context for the [`unhandled`] handler.
    ///
    /// [`unhandled`]: ../struct.Bot.html#method.unhandled
    struct Unhandled {
        /// The unhandled update.
        update: types::UpdateKind,
    }
}

impl Unhandled {
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
