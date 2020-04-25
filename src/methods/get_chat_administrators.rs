use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Gets information about a chat's admins.
///
/// Reflects the [`getChatAdministrators`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatadministrators
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatAdministrators<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a> GetChatAdministrators<'a> {
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

impl GetChatAdministrators<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<chat::Member>, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "getChatAdministrators",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
