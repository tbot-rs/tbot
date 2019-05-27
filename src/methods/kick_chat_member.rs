use super::*;

/// Represents the [`kickChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#kickchatmember
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct KickChatMember<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    user_id: i64,
    until_date: i64,
}

impl<'a> KickChatMember<'a> {
    /// Constructs a new `KickChatMember`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
        until_date: i64,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            user_id,
            until_date,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "kickChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ()) // Only `true` is returned on success
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for KickChatMember<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
