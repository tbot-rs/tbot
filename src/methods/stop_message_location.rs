use super::*;

/// Represents the [`stopMessageLiveLocation`][docs] method for chat messages.
///
/// [docs]: https://core.telegram.org/bots/api#stopmessagelivelocation
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopMessageLocation<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    message_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> StopMessageLocation<'a> {
    /// Constructs a new `StopMessageLocation`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> Self {
        Self {
            token,
            #[cfg(feature = "proxy")]
            proxy: None,
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(mut self, markup: types::InlineKeyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl IntoFuture for StopMessageLocation<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.token,
            "stopMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for StopMessageLocation<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
