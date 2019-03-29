use super::*;

/// Represents the [`deleteMessage`][docs] method for when the message was
/// sent by the bot.
///
/// [docs]: https://core.telegram.org/bots/api#deletemessage
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteMessage<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    message_id: u64,
}

impl<'a> DeleteMessage<'a> {
    /// Constructs a new `DeleteMessage`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u64,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            message_id,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method(
            self.token,
            "deleteMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for DeleteMessage<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
