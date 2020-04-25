use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        parameters::{ChatId, ImplicitChatId},
        Chat,
    },
};
use serde::Serialize;

/// Gets information about a chat.
///
/// Reflects the [`getChat`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchat
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChat<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a> GetChat<'a> {
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

impl GetChat<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Chat, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "getChat",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
