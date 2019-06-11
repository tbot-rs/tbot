use super::*;

/// Represents the [`editMessageReplyMarkup`][docs] method for inline messages.
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
}

impl IntoFuture for EditInlineReplyMarkup<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                self.token,
                "editMessageReplyMarkup",
                None,
                serde_json::to_vec(&self).unwrap(),
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()),
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for EditInlineReplyMarkup<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
