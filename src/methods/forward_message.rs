use super::*;

/// Represents the [`forwardMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#forwardmessage
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct ForwardMessage<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    from_chat_id: types::ChatId<'a>,
    message_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a> ForwardMessage<'a> {
    /// Constructs a new `ForwardMessage`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `disable_notification`.
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }
}

impl IntoFuture for ForwardMessage<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.token,
            "forwardMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for ForwardMessage<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
