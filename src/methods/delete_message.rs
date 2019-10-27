use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        message,
        parameters::{ChatId, ImplicitChatId},
    },
    Token,
};
use serde::Serialize;

/// Deletes a message from a chat.
///
/// Reflects the [`deleteMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#deletemessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteMessage<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    message_id: message::Id,
}

impl<'a, C> DeleteMessage<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl<C: Connector> DeleteMessage<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            &self.token,
            "deleteMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
