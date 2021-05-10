use crate::{
    types::{ChosenInlineResult, InlineMessageId, Location, User},
    Bot,
};

common! {
    /// The context for [`chosen_inline`] handlers.
    ///
    /// [`chosen_inline`]: crate::EventLoop::chosen_inline
    struct ChosenInline {
        /// ID of the chosen result.
        result_id: String,
        /// The user who chose the result.
        from: User,
        /// The location of the user, if enabled and allowed.
        location: Option<Location>,
        /// The ID of the sent message.
        inline_message_id: Option<InlineMessageId>,
        /// The query used to obtain the result.
        query: String,
    }
}

impl ChosenInline {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Bot, chosen_result: ChosenInlineResult) -> Self {
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
