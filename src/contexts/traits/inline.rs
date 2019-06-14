use crate::{
    methods::AnswerInlineQuery, types::inline_query_result::InlineQueryResult,
    types::InlineQueryId, Bot,
};

/// Provides methods appliable to callback queries.
pub trait Inline<'a, C: 'static>: crate::internal::Sealed {
    #[doc(hidden)]
    fn bot(&self) -> &Bot<C>;
    #[doc(hidden)]
    fn id(&self) -> &InlineQueryId;

    /// Opens a URL.
    fn answer(
        &'a self,
        results: Vec<InlineQueryResult<'a>>,
    ) -> AnswerInlineQuery<'a, C> {
        self.bot().answer_inline_query(self.id(), results)
    }
}
