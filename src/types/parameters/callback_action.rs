use crate::types::value;

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
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CallbackAction<'a> {
    /// No action.
    None,
    /// Show text to the user. The last item configures `show_alert`.
    Text(value::String<'a>, bool),
    /// Open a URL.
    Url(value::String<'a>),
}

impl<'a> CallbackAction<'a> {
    /// Constructs the `None` variant.
    pub const fn none() -> Self {
        CallbackAction::None
    }

    /// Constructs the `Text` variant that shows a simple notification.
    pub fn notification(text: impl Into<value::String<'a>>) -> Self {
        CallbackAction::Text(text.into(), false)
    }

    /// Constructs the `Text` variant that shows an alert.
    pub fn alert(text: impl Into<value::String<'a>>) -> Self {
        CallbackAction::Text(text.into(), true)
    }

    /// Constructs the `Url` variant.
    pub fn url(url: impl Into<value::String<'a>>) -> Self {
        CallbackAction::Url(url.into())
    }
}
