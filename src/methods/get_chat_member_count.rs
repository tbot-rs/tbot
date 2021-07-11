use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Gets a chat's member count.
///
/// Reflects the [`getChatMembersCount`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmemberscount
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMemberCount<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
}

impl<'a> GetChatMemberCount<'a> {
    pub(crate) fn new(bot: &'a InnerBot, chat_id: impl ImplicitChatId) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl GetChatMemberCount<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<u32, errors::MethodCall> {
        call_method(
            self.bot,
            "getChatMemberCount",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
