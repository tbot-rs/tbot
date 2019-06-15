use crate::Bot;
use types::{ChosenInlineResult, Location, User};

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
        inline_message_id: Option<String>,
        /// The query used to obtain the result.
        query: String,
    }
}

impl<C> ChosenInline<C> {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: Arc<Bot<C>>,
        inline_query: ChosenInlineResult,
    ) -> Self {
        Self {
            bot,
            result_id: inline_query.result_id,
            from: inline_query.from,
            location: inline_query.location,
            inline_message_id: inline_query.inline_message_id,
            query: inline_query.query,
        }
    }
}
