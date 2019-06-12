use super::*;

/// Represents the [`deleteMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#deletemessage
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteMessage<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    message_id: u32,
}

impl<'a> DeleteMessage<'a> {
    /// Constructs a new `DeleteMessage`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            message_id,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for DeleteMessage<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                &self.token,
                "deleteMessage",
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
impl ProxyMethod for DeleteMessage<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
