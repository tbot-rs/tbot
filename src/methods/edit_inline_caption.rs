use super::*;

/// Represents the [`editMessageCaption`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagecaption
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineCaption<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    inline_message_id: &'a str,
    caption: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<types::ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> EditInlineCaption<'a> {
    /// Constructs a new `EditInlineCaption`.
    pub const fn new(
        token: &'a str,
        inline_message_id: &'a str,
        caption: &'a str,
    ) -> Self {
        Self {
            token,
            inline_message_id,
            caption,
            parse_mode: None,
            reply_markup: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, mode: types::ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl IntoFuture for EditInlineCaption<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                self.token,
                "editMessageCaption",
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
impl ProxyMethod for EditInlineCaption<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
