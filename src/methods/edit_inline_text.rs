use super::*;

/// Represents the [`editMessageText`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagetext
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineText<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    inline_message_id: &'a str,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<types::ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> EditInlineText<'a> {
    /// Constructs a new `EditMessageText`.
    pub const fn new(
        token: Token,
        inline_message_id: &'a str,
        text: &'a str,
    ) -> Self {
        Self {
            token,
            inline_message_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
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

    /// Configures `disable_web_page_preview`.
    pub fn disable_web_page_preview(mut self, is_disabled: bool) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl IntoFuture for EditInlineText<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                &self.token,
                "editMessageText",
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
impl ProxyMethod for EditInlineText<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
