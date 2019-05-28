use super::*;

/// Represents the [`getChatAdministrators`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatadministrators
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatAdministrators<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
}

impl<'a> GetChatAdministrators<'a> {
    /// Constructs a new `GetChatAdministrators`.
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
    pub fn into_future(
        self,
    ) -> impl Future<Item = Vec<types::raw::ChatMember>, Error = DeliveryError>
    {
        send_method(
            self.token,
            "getChatAdministrators",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetChatAdministrators<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
