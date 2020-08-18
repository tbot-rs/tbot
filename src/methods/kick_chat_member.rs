use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};
use serde::Serialize;

/// Kicks a member out of a chat.
///
/// Reflects the [`kickChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#kickchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct KickChatMember<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    user_id: user::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
}

impl<'a> KickChatMember<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
        }
    }

    /// Configures when the user regains the ability to return to the chat.
    /// Reflects the `until_date` parameter.
    pub const fn until_date(mut self, date: i64) -> Self {
        self.until_date = Some(date);
        self
    }
}

impl KickChatMember<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "kickChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
