use crate::{
    methods::{AnswerCallbackQuery, CallbackAnswerAction, Methods},
    MockBot,
};

/// Provides methods appliable to callback queries.
pub trait Callback<'a>: crate::internal::Sealed {
    #[doc(hidden)]
    fn bot(&self) -> &MockBot;
    #[doc(hidden)]
    fn id(&self) -> &str;

    /// Opens a URL.
    fn open_url(&'a self, url: &'a str) -> AnswerCallbackQuery<'a> {
        self.bot()
            .answer_callback_query(self.id(), CallbackAnswerAction::url(url))
    }

    /// Shows a notification to the user.
    fn notify(&'a self, text: &'a str) -> AnswerCallbackQuery<'a> {
        self.bot().answer_callback_query(
            self.id(),
            CallbackAnswerAction::notification(text),
        )
    }

    /// Shows an alert to the user.
    fn alert(&'a self, text: &'a str) -> AnswerCallbackQuery<'a> {
        self.bot()
            .answer_callback_query(self.id(), CallbackAnswerAction::alert(text))
    }
}
