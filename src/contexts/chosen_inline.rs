use crate::{
    types::{ChosenInlineResult, InlineMessageId, Location, User},
    Bot,
};
use std::sync::Arc;

common! {
    /// The context for [`chosen_inline`][handler] handlers.
    ///
    /// [handler]: ../event_loop/struct.EventLoop.html#method.chosen_inline
    struct ChosenInline {
        /// ID of the chosen result.
        result_id: String,
        /// The user who chose the result.
        from: User,
        /// The location of the user, if enabled and allowed.
        location: Option<Location>,
        /// The ID of the sent message.
        inline_message_id: Option<InlineMessageId<'static>>,
        /// The query used to obtain the result.
        query: String,
    }
}

impl ChosenInline {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: Arc<Bot>,
        chosen_result: ChosenInlineResult,
    ) -> Self {
        Self {
            bot,
            result_id: chosen_result.result_id,
            from: chosen_result.from,
            location: chosen_result.location,
            inline_message_id: chosen_result.inline_message_id,
            query: chosen_result.query,
        }
    }
}
