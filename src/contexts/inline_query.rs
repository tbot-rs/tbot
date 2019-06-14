common! {
    /// The context for [`inline_query`][handler] handlers.
    ///
    /// [handler]: ../struct.Bot.html#method.inline_query
    struct InlineQuery {
        /// The ID of the query.
        id: types::InlineQueryId,
        /// The user who sent the query.
        from: types::User,
        /// The location of the user, if enabled and allowed.
        location: Option<types::Location>,
        /// The query itself.
        query: String,
        /// The offset of the result to be returned.
        offset: String,
    }
}

impl<C> InlineQuery<C> {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: Arc<Bot<C>>,
        inline_query: types::InlineQuery,
    ) -> Self {
        Self {
            bot,
            id: inline_query.id,
            from: inline_query.from,
            location: inline_query.location,
            query: inline_query.query,
            offset: inline_query.offset,
        }
    }
}

impl<'a, C: 'static> traits::Inline<'a, C> for InlineQuery<C> {
    fn bot(&self) -> &Bot<C> {
        &self.bot
    }

    fn id(&self) -> &types::InlineQueryId {
        &self.id
    }
}
