use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::parameters::{ChatId, ImplicitChatId},
    Token,
};
use serde::Serialize;

/// Exports a chat's invite link.
///
/// Reflects the [`exportChatInviteLink`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#exportchatinvitelink
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct ExportChatInviteLink<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
}

impl<'a, C> ExportChatInviteLink<'a, C> {
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

impl<C: Connector> ExportChatInviteLink<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<String, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "exportChatInviteLink",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
