use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::parameters::{ChatId, ImplicitChatId},
};

/// Gets a chat's member count.
///
/// Reflects the [`getChatMembersCount`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmemberscount
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMembersCount<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
}

impl<'a, C> GetChatMembersCount<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
        }
    }
}

impl<C: Connector> GetChatMembersCount<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<u32, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "getChatMembersCount",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
