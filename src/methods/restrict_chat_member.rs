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

/// Restricts a chat member.
///
/// Reflects the [`restrictChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#restrictchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct RestrictChatMember<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    user_id: user::Id,
    permissions: chat::Permissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
}

impl<'a> RestrictChatMember<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
        permissions: chat::Permissions,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            permissions,
            until_date: None,
        }
    }

    /// Configures when the restrictions will be lifted.
    /// Reflects the `until_date` parameter.
    pub const fn until_date(mut self, date: i64) -> Self {
        self.until_date = Some(date);
        self
    }
}

impl RestrictChatMember<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "restrictChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
