use is_macro::Is;

/// Represent possible actions for [`AnswerCallbackQuery`].
///
/// Though you can consturct variants directly, there are convenient methods
/// to do that: [`with_notification`], [`with_alert`] and [`with_url`].
///
/// [`AnswerCallbackQuery`]: ./struct.AnswerCallbackQuery.html
/// [`with_notification`]: #method.with_notification
/// [`with_alert`]: #method.with_alert
/// [`with_url`]: #method.with_url
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[must_use]
pub enum CallbackAction<'a> {
    /// Show text to the user. The last item configures `show_alert`.
    Text(&'a str, bool),
    /// Open a URL.
    Url(&'a str),
}

impl<'a> CallbackAction<'a> {
    /// Constructs the `Text` variant that shows a simple notification.
    pub const fn with_notification(text: &'a str) -> Self {
        CallbackAction::Text(text, false)
    }

    /// Constructs the `Text` variant that shows an alert.
    pub const fn with_alert(text: &'a str) -> Self {
        CallbackAction::Text(text, true)
    }

    /// Constructs the `Url` variant.
    pub const fn with_url(url: &'a str) -> Self {
        CallbackAction::Url(url)
    }

    pub(crate) const fn to_text(self) -> Option<&'a str> {
        match self {
            CallbackAction::Text(text, _) => Some(text),
            CallbackAction::Url(_) => None,
        }
    }

    pub(crate) const fn to_show_alert(self) -> Option<bool> {
        match self {
            CallbackAction::Text(_, should_show) => Some(should_show),
            CallbackAction::Url(_) => None,
        }
    }

    pub(crate) const fn to_url(self) -> Option<&'a str> {
        match self {
            CallbackAction::Url(url) => Some(url),
            CallbackAction::Text(..) => None,
        }
    }
}
