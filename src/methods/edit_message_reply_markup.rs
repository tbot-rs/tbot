use super::*;

/// Represents the [`editMessageReplyMarkup`][docs] method for chat messsages.
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
    message_id: u32,
    reply_markup: types::InlineKeyboard<'a>,
}

impl<'a> EditMessageReplyMarkup<'a> {
    /// Constructs a new `EditMessageReplyMarkup`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
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
}

impl IntoFuture for EditMessageReplyMarkup<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.token,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for EditMessageReplyMarkup<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
