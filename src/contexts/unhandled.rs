common! {
    /// The context for [`unhandled`] handlers.
    ///
    /// [`unhandled`]: ../struct.Bot.html#method.unhandled
    struct Unhandled {
        /// The unhandled update.
        update: types::UpdateKind,
    }
}

impl<C> Unhandled<C> {
    pub(crate) const fn new(
        bot: Arc<Bot<C>>,
        update: types::UpdateKind,
    ) -> Self {
        Self {
            bot,
            update,
        }
    }
}
