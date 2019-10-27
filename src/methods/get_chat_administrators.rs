use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
    Token,
};
use serde::Serialize;

/// Gets information about a chat's admins.
///
/// Reflects the [`getChatAdministrators`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatadministrators
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatAdministrators<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
}

impl<'a, C> GetChatAdministrators<'a, C> {
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

impl<C: Connector> GetChatAdministrators<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<chat::Member>, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "getChatAdministrators",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
