
common! {
    /// The context for [`before_update`] and [`after_update`] handlers.
    ///
    /// This context does _not_ provide the contents of the update. Use dedicated
    /// update handlers instead.
    ///
    /// [`before_update`]: ../struct.Bot.html#method.before_update
    /// [`after_update`]: ../struct.Bot.html#method.after_update
    struct Update {
        /// The ID of the update.
        update_id: u32,
    }
}


impl<C> Update<C> {
    pub(crate) const fn new(bot: Arc<Bot<C>>, update_id: u32) -> Self {
        Self {
            bot,
            update_id,
        }
    }
}
