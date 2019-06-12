use super::*;

/// Represents the [`kickChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#kickchatmember
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct KickChatMember<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
}

impl<'a> KickChatMember<'a> {
    /// Constructs a new `KickChatMember`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `until_date`.
    pub fn until_date(mut self, date: i64) -> Self {
        self.until_date = Some(date);
        self
    }
}

impl IntoFuture for KickChatMember<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                &self.token,
                "kickChatMember",
                None,
                serde_json::to_vec(&self).unwrap(),
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()), // Only `true` is returned on successы
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for KickChatMember<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
