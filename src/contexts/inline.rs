use crate::{
    methods::AnswerInlineQuery,
    types::{inline_query, InlineQuery, Location, User},
    Bot,
};

common! {
    /// The context for [`inline`] handlers.
    ///
    /// [`inline`]: crate::EventLoop::inline
    struct Inline {
        /// The ID of the query.
        id: inline_query::Id,
        /// The user who sent the query.
        from: User,
        /// The location of the user, if enabled and allowed.
        location: Option<Location>,
        /// The query itself.
        query: String,
        /// The offset of the result to be returned.
        offset: String,
        /// The type of chat inline query was sent from.
        chat_kind: Option<inline_query::ChatKind>,
    }
}

impl Inline {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Bot, inline_query: InlineQuery) -> Self {
        Self {
            bot,
            id: inline_query.id,
            from: inline_query.from,
            location: inline_query.location,
            query: inline_query.query,
            offset: inline_query.offset,
            chat_kind: inline_query.chat_kind,
        }
    }

    /// Answers the query.
    pub fn answer(
        &self,
        results: impl Into<Vec<inline_query::Result>>,
    ) -> AnswerInlineQuery<'_> {
        self.bot.answer_inline_query(self.id.clone(), results)
    }
}
