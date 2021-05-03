use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};
use serde::Serialize;

/// Gets information about a chat's member.
///
/// Reflects the [`getChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMember<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    user_id: user::Id,
}

impl<'a> GetChatMember<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl GetChatMember<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<chat::Member, errors::MethodCall> {
        call_method(
            self.bot,
            "getChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
