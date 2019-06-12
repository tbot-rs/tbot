use super::*;

/// Represents the [`stopMessageLiveLocation`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopInlineLocation<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    inline_message_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> StopInlineLocation<'a> {
    /// Constructs a new `StopInlineLocation`.
    pub const fn new(token: Token, inline_message_id: &'a str) -> Self {
        Self {
            token,
            inline_message_id,
            reply_markup: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl IntoFuture for StopInlineLocation<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                &self.token,
                "stopMessageLiveLocation",
                None,
                serde_json::to_vec(&self).unwrap(),
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for StopInlineLocation<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
