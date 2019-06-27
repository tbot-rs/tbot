use crate::{
    methods::AnswerCallbackQuery,
    types::{callback, parameters::CallbackAction},
    Bot,
};

/// Provides methods appliable to callback queries.
pub trait Callback<'a, C: 'static>: crate::internal::Sealed {
    #[doc(hidden)]
    fn bot(&self) -> &Bot<C>;
    #[doc(hidden)]
    fn id(&self) -> callback::query::id::Ref<'_>;

    /// Opens a URL.
    fn open_url(&'a self, url: &'a str) -> AnswerCallbackQuery<'a, C> {
        self.bot().answer_callback_query(self.id(), CallbackAction::url(url))
    }

    /// Shows a notification to the user.
    fn notify(&'a self, text: &'a str) -> AnswerCallbackQuery<'a, C> {
        self.bot().answer_callback_query(
            self.id(),
            CallbackAction::notification(text),
        )
    }

    /// Shows an alert to the user.
    fn alert(&'a self, text: &'a str) -> AnswerCallbackQuery<'a, C> {
        self.bot().answer_callback_query(self.id(), CallbackAction::alert(text))
    }
}
