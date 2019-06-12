use super::*;

/// Represents the [`setChatTitle`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchattitle
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatTitle<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    title: &'a str,
}

impl<'a> SetChatTitle<'a> {
    /// Constructs a new `SetChatTitle`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        title: &'a str,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            title,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for SetChatTitle<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                self.token,
                "setChatTitle",
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
impl ProxyMethod for SetChatTitle<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
