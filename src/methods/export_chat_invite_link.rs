use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
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
    bot: &'a InnerBot,
    chat_id: ChatId,
}

impl<'a> ExportChatInviteLink<'a> {
    pub(crate) fn new(bot: &'a InnerBot, chat_id: impl ImplicitChatId) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl ExportChatInviteLink<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<String, errors::MethodCall> {
        call_method(
            self.bot,
            "exportChatInviteLink",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
