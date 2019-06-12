use super::*;

/// Represents the [`setChatDescription`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatdescription
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatDescription<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    description: &'a str,
}

impl<'a> SetChatDescription<'a> {
    /// Constructs a new `SetChatDescription`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        description: &'a str,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            description,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for SetChatDescription<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                &self.token,
                "setChatDescription",
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
impl ProxyMethod for SetChatDescription<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
