use super::*;

/// Represents the [`getChatMembersCount`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmemberscount
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMembersCount<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
}

impl<'a> GetChatMembersCount<'a> {
    /// Constructs a new `GetChatMembersCount`.
    pub fn new(token: &'a str, chat_id: impl Into<types::ChatId<'a>>) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = u32, Error = DeliveryError> {
        send_method(
            self.token,
            "getChatMembersCount",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetChatMembersCount<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
