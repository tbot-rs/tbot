use super::*;

/// Represents the [`getChatMembersCount`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmemberscount
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMembersCount<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
}

impl<'a> GetChatMembersCount<'a> {
    /// Constructs a new `GetChatMembersCount`.
    pub fn new(token: Token, chat_id: impl Into<types::ChatId<'a>>) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for GetChatMembersCount<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = u32;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.token,
            "getChatMembersCount",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetChatMembersCount<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
