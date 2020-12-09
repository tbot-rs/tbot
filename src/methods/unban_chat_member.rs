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

/// Lifts all restrictions from a group's member.
///
/// Reflects the [`unbanChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#unbanchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UnbanChatMember<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    user_id: user::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_if_banned: Option<bool>,
}

impl<'a> UnbanChatMember<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            only_if_banned: None,
        }
    }
}

impl UnbanChatMember<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            &*self.bot,
            "unbanChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }

    /// Configures banning rule, tells to do nothing if the user is not banned.
    /// Reflects the `only_if_banned` parameter.
    pub const fn only_if_banned(mut self, only_if_banned: bool) -> Self {
        self.only_if_banned = Some(only_if_banned);
        self
    }
}
