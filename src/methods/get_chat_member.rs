use super::*;

/// Represents the [`getChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmember
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMember<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    user_id: i64,
}

impl<'a> GetChatMember<'a> {
    /// Constructs a new `GetChatMember`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            user_id,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for GetChatMember<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::ChatMember;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.token,
            "getChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetChatMember<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
