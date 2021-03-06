use is_macro::Is;

/// Represent possible actions for [`AnswerCallbackQuery`].
///
/// Though you can consturct variants directly, there are convenient methods
/// to do that: [`with_notification`], [`with_alert`] and [`with_url`].
///
/// [`AnswerCallbackQuery`]: crate::methods::AnswerCallbackQuery
/// [`with_notification`]: Self::with_notification
/// [`with_alert`]: Self::with_alert
/// [`with_url`]: Self::with_url
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[must_use]
pub enum CallbackAction {
    /// Show text to the user. The last item configures `show_alert`.
    Text(String, bool),
    /// Open a URL.
    Url(String),
}

impl CallbackAction {
    /// Constructs the `Text` variant that shows a simple notification.
    pub fn with_notification(text: impl Into<String>) -> Self {
        Self::Text(text.into(), false)
    }

    /// Constructs the `Text` variant that shows an alert.
    pub fn with_alert(text: impl Into<String>) -> Self {
        Self::Text(text.into(), true)
    }

    /// Constructs the `Url` variant.
    pub fn with_url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }
}
