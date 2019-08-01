use crate::{
    methods::AnswerCallbackQuery,
    types::{
        callback,
        parameters::CallbackAction,
        value::{self, Ref},
    },
    Bot,
};

/// Provides methods appliable to callback queries.
pub trait Callback<'a, C: 'static>: crate::internal::Sealed {
    #[doc(hidden)]
    fn bot(&self) -> &Bot<C>;
    #[doc(hidden)]
    fn id(&self) -> callback::query::id::Ref<'_>;

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
    fn answer(
        &'a self,
        action: impl Into<Ref<'a, CallbackAction<'a>>>,
    ) -> AnswerCallbackQuery<'a, C> {
        self.bot().answer_callback_query(self.id(), action)
    }

    /// Answers the query without any action.
    fn ignore(&'a self) -> AnswerCallbackQuery<'a, C> {
        self.answer(CallbackAction::none())
    }

    /// Opens a URL.
    fn open_url(
        &'a self,
        url: impl Into<value::String<'a>>,
    ) -> AnswerCallbackQuery<'a, C> {
        self.answer(CallbackAction::url(url.into()))
    }

    /// Shows a notification to the user.
    fn notify(
        &'a self,
        text: impl Into<value::String<'a>>,
    ) -> AnswerCallbackQuery<'a, C> {
        self.answer(CallbackAction::notification(text.into()))
    }

    /// Shows an alert to the user.
    fn alert(
        &'a self,
        text: impl Into<value::String<'a>>,
    ) -> AnswerCallbackQuery<'a, C> {
        self.answer(CallbackAction::alert(text.into()))
    }
}
