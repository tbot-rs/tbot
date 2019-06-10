use super::*;

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
pub enum CallbackAnswerAction<'a> {
    /// No action.
    None,
    /// Show text to the user. The last item configures `show_alert`.
    Text(&'a str, bool),
    /// Open a URL.
    Url(&'a str),
}

impl<'a> CallbackAnswerAction<'a> {
    /// Constructs the `None` variant.
    pub const fn none() -> Self {
        CallbackAnswerAction::None
    }

    /// Constructs the `Text` variant that shows a simple notification.
    pub fn notification(text: &'a str) -> Self {
        CallbackAnswerAction::Text(text, false)
    }

    /// Constructs the `Text` variant that shows an alert.
    pub fn alert(text: &'a str) -> Self {
        CallbackAnswerAction::Text(text, true)
    }

    /// Constructs the `Url` variant.
    pub fn url(url: &'a str) -> Self {
        CallbackAnswerAction::Url(url)
    }

    fn to_text(self) -> Option<&'a str> {
        match self {
            CallbackAnswerAction::Text(text, _) => Some(text),
            _ => None,
        }
    }

    fn to_show_alert(self) -> Option<bool> {
        match self {
            CallbackAnswerAction::Text(_, should_show) => Some(should_show),
            _ => None,
        }
    }

    fn to_url(self) -> Option<&'a str> {
        match self {
            CallbackAnswerAction::Url(url) => Some(url),
            _ => None,
        }
    }
}

/// Represents the [`answerCallbackQuery`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#answercallbackquery
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AnswerCallbackQuery<'a> {
    #[serde(skip)]
    token: &'a str,
    callback_query_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u32>,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
}

impl<'a> AnswerCallbackQuery<'a> {
    /// Constructs a new `AnswerCallbackQuery`.
    pub fn new(
        token: &'a str,
        callback_query_id: &'a str,
        action: CallbackAnswerAction<'a>,
    ) -> Self {
        Self {
            token,
            callback_query_id,
            text: action.to_text(),
            show_alert: action.to_show_alert(),
            url: action.to_url(),
            cache_time: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `cache_time`.
    pub fn cache_time(mut self, time: u32) -> Self {
        self.cache_time = Some(time);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "answerCallbackQuery",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ()) // Only `true` is returned on success
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for AnswerCallbackQuery<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
