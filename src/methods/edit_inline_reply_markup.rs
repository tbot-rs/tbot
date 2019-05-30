use super::*;

/// Represents the [`editInlineReplyMarkup`][docs] method for when the message
/// was sent by the bot via the inline mode.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineReplyMarkup<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    inline_message_id: &'a str,
    reply_markup: types::InlineKeyboard<'a>,
}

impl<'a> EditInlineReplyMarkup<'a> {
    /// Constructs a new `EditInlineReplyMarkup`.
    pub const fn new(
        token: &'a str,
        inline_message_id: &'a str,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> Self {
        Self {
            token,
            inline_message_id,
            reply_markup,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ())
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for EditInlineReplyMarkup<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
