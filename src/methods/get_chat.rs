use super::*;

/// Represents the [`getChat`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchat
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChat<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
}

impl<'a> GetChat<'a> {
    /// Constructs a new `GetChat`.
    pub fn new(token: &'a str, chat_id: impl Into<types::ChatId<'a>>) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::Chat, Error = DeliveryError> {
        send_method(
            self.token,
            "getChat",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetChat<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
