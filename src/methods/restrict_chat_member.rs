use super::*;

/// Represents the [`restrictChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#restrictchatmember
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct RestrictChatMember<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_media_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_other_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_add_web_page_previews: Option<bool>,
}

impl<'a> RestrictChatMember<'a> {
    /// Constructs a new `RestrictChatMember`.
    pub fn new(
        token: &'a str,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `until_date`.
    pub fn until_date(mut self, date: i64) -> Self {
        self.until_date = Some(date);
        self
    }

    /// Configures `can_send_messages`.
    pub fn can_send_messages(mut self, can_send: bool) -> Self {
        self.can_send_messages = Some(can_send);
        self
    }

    /// Configures `can_send_media_messages`.
    pub fn can_send_media_messages(mut self, can_send: bool) -> Self {
        self.can_send_media_messages = Some(can_send);
        self
    }

    /// Configures `can_send_other_messages`.
    pub fn can_send_other_messages(mut self, can_send: bool) -> Self {
        self.can_send_other_messages = Some(can_send);
        self
    }

    /// Configures `can_add_web_page_previews`.
    pub fn can_add_web_page_previews(mut self, can_add: bool) -> Self {
        self.can_add_web_page_previews = Some(can_add);
        self
    }
}

impl IntoFuture for RestrictChatMember<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                self.token,
                "restrictChatMember",
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
impl ProxyMethod for RestrictChatMember<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
