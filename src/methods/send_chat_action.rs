use super::*;

/// Represents the [`sendChatAction`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendchataction
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendChatAction<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    action: types::ChatAction,
}

impl<'a> SendChatAction<'a> {
    /// Constructs a new `SendChatAction`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        action: types::ChatAction,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            action,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "sendChatAction",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        // The only value `true` is returned on success.
        .map(|_| ())
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SendChatAction<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
