use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Revokes an invite link for a chat.
///
/// Reflects the [`revokeChatInviteLink`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#revokechatinvitelink
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct RevokeChatInviteLink<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    invite_link: String,
}

impl<'a> RevokeChatInviteLink<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        link: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            invite_link: link.into(),
        }
    }
}

impl RevokeChatInviteLink<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<chat::InviteLink, errors::MethodCall> {
        call_method(
            self.bot,
            "revokeChatInviteLink",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
