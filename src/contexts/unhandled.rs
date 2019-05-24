common! {
    /// Context for the [`unhandled`] handler.
    ///
    /// [`unhandled`]: ../struct.Bot.html#method.unhandled
    struct UnhandledContext {
        /// The unhandled update.
        update: types::UpdateKind,
    }
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
