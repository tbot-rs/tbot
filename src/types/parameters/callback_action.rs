use is_macro::Is;
use std::borrow::Cow;

/// Represent possible actions for [`AnswerCallbackQuery`].
///
/// Though you can consturct variants directly, there are convenient methods
/// to do that: [`with_notification`], [`with_alert`] and [`with_url`].
///
/// [`AnswerCallbackQuery`]: ./struct.AnswerCallbackQuery.html
/// [`with_notification`]: #method.with_notification
/// [`with_alert`]: #method.with_alert
/// [`with_url`]: #method.with_url
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[must_use]
pub enum CallbackAction<'a> {
    /// Show text to the user. The last item configures `show_alert`.
    Text(Cow<'a, str>, bool),
    /// Open a URL.
    Url(Cow<'a, str>),
}

impl<'a> CallbackAction<'a> {
    /// Constructs the `Text` variant that shows a simple notification.
    pub fn with_notification(text: impl Into<Cow<'a, str>>) -> Self {
        CallbackAction::Text(text.into(), false)
    }

    /// Constructs the `Text` variant that shows an alert.
    pub fn with_alert(text: impl Into<Cow<'a, str>>) -> Self {
        CallbackAction::Text(text.into(), true)
    }

    /// Constructs the `Url` variant.
    pub fn with_url(url: impl Into<Cow<'a, str>>) -> Self {
        CallbackAction::Url(url.into())
    }
}
