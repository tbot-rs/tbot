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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum CallbackAction<'a> {
    /// No action.
    None,
    /// Show text to the user. The last item configures `show_alert`.
    Text(&'a str, bool),
    /// Open a URL.
    Url(&'a str),
}

impl<'a> CallbackAction<'a> {
    /// Constructs the `None` variant.
    pub const fn none() -> Self {
        CallbackAction::None
    }

    /// Constructs the `Text` variant that shows a simple notification.
    pub fn notification(text: &'a str) -> Self {
        CallbackAction::Text(text, false)
    }

    /// Constructs the `Text` variant that shows an alert.
    pub fn alert(text: &'a str) -> Self {
        CallbackAction::Text(text, true)
    }

    /// Constructs the `Url` variant.
    pub fn url(url: &'a str) -> Self {
        CallbackAction::Url(url)
    }

    pub(crate) fn to_text(self) -> Option<&'a str> {
        match self {
            CallbackAction::Text(text, _) => Some(text),
            _ => None,
        }
    }

    pub(crate) fn to_show_alert(self) -> Option<bool> {
        match self {
            CallbackAction::Text(_, should_show) => Some(should_show),
            _ => None,
        }
    }

    pub(crate) fn to_url(self) -> Option<&'a str> {
        match self {
            CallbackAction::Url(url) => Some(url),
            _ => None,
        }
    }
}
