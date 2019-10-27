use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    token,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};
use serde::Serialize;

/// Gets information about a chat's member.
///
/// Reflects the [`getChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMember<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    user_id: user::Id,
}

impl<'a, C> GetChatMember<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl<C: Connector> GetChatMember<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<chat::Member, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "getChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
