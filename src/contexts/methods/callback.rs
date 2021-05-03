use crate::{
    contexts::fields, methods::AnswerCallbackQuery,
    types::parameters::CallbackAction,
};
use std::borrow::Cow;

/// Provides methods appliable to callback queries.
pub trait Callback: fields::Callback {
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
        action: Option<CallbackAction<'a>>,
    ) -> AnswerCallbackQuery<'a> {
        self.bot().answer_callback_query(self.id().clone(), action)
    }

    /// Answers the query without any action.
    fn ignore(&self) -> AnswerCallbackQuery<'_> {
        self.answer(None)
    }

    /// Opens a URL.
    fn open_url<'a>(
        &'a self,
        url: impl Into<Cow<'a, str>>,
    ) -> AnswerCallbackQuery<'a> {
        self.answer(Some(CallbackAction::with_url(url)))
    }

    /// Shows a notification to the user.
    fn notify<'a>(
        &'a self,
        text: impl Into<Cow<'a, str>>,
    ) -> AnswerCallbackQuery<'a> {
        self.answer(Some(CallbackAction::with_notification(text)))
    }

    /// Shows an alert to the user.
    fn alert<'a>(
        &'a self,
        text: impl Into<Cow<'a, str>>,
    ) -> AnswerCallbackQuery<'a> {
        self.answer(Some(CallbackAction::with_alert(text)))
    }
}

impl<T: fields::Callback> Callback for T {}
