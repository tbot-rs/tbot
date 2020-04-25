use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Sends a chat action.
///
/// Reflects the [`sendChatAction`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendchataction
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendChatAction<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    action: chat::Action,
}

impl<'a> SendChatAction<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
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

impl SendChatAction<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool>(
            self.client,
            self.token,
            "sendChatAction",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
