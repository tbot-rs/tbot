use super::*;

/// Represents the [`EditMessageReplyMarkup`][docs] method for when the
/// message was sent by the bot.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageReplyMarkup<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    message_id: u64,
    reply_markup: types::InlineKeyboard<'a>,
}

impl<'a> EditMessageReplyMarkup<'a> {
    /// Constructs a new `EditMessageReplyMarkup`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u64,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            message_id,
            reply_markup,
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
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for EditMessageReplyMarkup<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
