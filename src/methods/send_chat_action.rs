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

/// Sends a chat action.
///
/// Reflects the [`sendChatAction`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendchataction
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendChatAction<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    action: chat::Action,
}

impl<'a, C> SendChatAction<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        action: chat::Action,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            action,
        }
    }
}

impl<C: Connector> SendChatAction<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            &self.token,
            "sendChatAction",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
