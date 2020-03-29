use crate::{
    contexts::fields, methods::AnswerCallbackQuery,
    types::parameters::CallbackAction,
};

/// Provides methods appliable to callback queries.
pub trait Callback<'b, C: 'static>: fields::Callback<C> {
    /// Answers the callback query.
    ///
    /// If you don't need to choose the action dynamically, using dedicated
    /// methods will be more convenient: [`ignore`], [`open_url`], [`notify`]
    /// and [`alert`].
    ///
    /// [`ignore`]: #method.ignore
    /// [`open_url`]: #method.open_url
    /// [`notify`]: #method.notify
    /// [`alert`]: #method.alert
    fn answer<'a>(
        &'a self,
        action: CallbackAction<'a>,
    ) -> AnswerCallbackQuery<'a, C> {
        self.bot().answer_callback_query(self.id().as_ref(), action)
    }

    /// Answers the query without any action.
    fn ignore(&self) -> AnswerCallbackQuery<'_, C> {
        self.answer(CallbackAction::with_no_action())
    }

    /// Opens a URL.
    fn open_url<'a>(&'a self, url: &'a str) -> AnswerCallbackQuery<'a, C> {
        self.answer(CallbackAction::with_url(url))
    }

    /// Shows a notification to the user.
    fn notify<'a>(&'a self, text: &'a str) -> AnswerCallbackQuery<'a, C> {
        self.answer(CallbackAction::with_notification(text))
    }

    /// Shows an alert to the user.
    fn alert<'a>(&'a self, text: &'a str) -> AnswerCallbackQuery<'a, C> {
        self.answer(CallbackAction::with_alert(text))
    }
}

impl<'a, C: 'static, T: fields::Callback<C>> Callback<'a, C> for T {}
