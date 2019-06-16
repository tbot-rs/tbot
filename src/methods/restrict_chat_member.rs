use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::parameters::ChatId,
};

/// Represents the [`restrictChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#restrictchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct RestrictChatMember<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
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

impl<'a, C> RestrictChatMember<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<ChatId<'a>>,
        user_id: i64,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
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

impl<C> IntoFuture for RestrictChatMember<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "restrictChatMember",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
