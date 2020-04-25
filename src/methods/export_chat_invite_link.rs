use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Exports a chat's invite link.
///
/// Reflects the [`exportChatInviteLink`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#exportchatinvitelink
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct ExportChatInviteLink<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a> ExportChatInviteLink<'a> {
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

impl ExportChatInviteLink<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<String, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "exportChatInviteLink",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
