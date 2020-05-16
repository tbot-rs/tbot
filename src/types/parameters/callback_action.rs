use is_macro::Is;
use std::borrow::Cow;

/// Represent possible actions for [`AnswerCallbackQuery`].
///
/// Though you can consturct variants directly, there are convenient methods
/// to do that: [`none`], [`notification`], [`alert`], [`url`].
///
/// [`AnswerCallbackQuery`]: ./struct.AnswerCallbackQuery.html
/// [`none`]: #method.none
/// [`notification`]: #method.notification
/// [`alert`]: #method.alert
/// [`url`]: #method.url
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[must_use]
pub enum CallbackAction<'a> {
    /// No action.
    None,
    /// Show text to the user. The last item configures `show_alert`.
    Text(Cow<'a, str>, bool),
    /// Open a URL.
    Url(Cow<'a, str>),
}

impl<'a> CallbackAction<'a> {
    /// Constructs the `None` variant.
    pub const fn with_no_action() -> Self {
        CallbackAction::None
    }

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
