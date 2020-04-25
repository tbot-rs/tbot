use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Gets a chat's member count.
///
/// Reflects the [`getChatMembersCount`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmemberscount
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMembersCount<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a> GetChatMembersCount<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
        }
    }
}

impl GetChatMembersCount<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<u32, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "getChatMembersCount",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
