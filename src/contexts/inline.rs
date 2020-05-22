use crate::{
    methods::AnswerInlineQuery,
    types::{inline_query, InlineQuery, Location, User},
    Bot,
};
use std::sync::Arc;

common! {
    /// The context for [`inline`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.inline
    struct Inline {
        /// The ID of the query.
        id: inline_query::Id<'static>,
        /// The user who sent the query.
        from: User,
        /// The location of the user, if enabled and allowed.
        location: Option<Location>,
        /// The query itself.
        query: String,
        /// The offset of the result to be returned.
        offset: String,
    }
}

impl Inline {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Arc<Bot>, inline_query: InlineQuery) -> Self {
        Self {
            bot,
            id: inline_query.id,
            from: inline_query.from,
            location: inline_query.location,
            query: inline_query.query,
            offset: inline_query.offset,
        }
    }

    /// Answers the query.
    pub fn answer<'a>(
        &'a self,
        results: &'a [inline_query::Result<'a>],
    ) -> AnswerInlineQuery<'a> {
        self.bot.answer_inline_query(self.id.as_ref(), results)
    }
}
