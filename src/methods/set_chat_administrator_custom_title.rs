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

/// Sets a custom title for an admin in a supergroup promoted by the bot.
///
/// Reflects the [`setChatAdministratorCustomTitle`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatadministratorcustomtitle
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatAdministratorCustomTitle<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    user_id: user::Id,
    custom_title: String,
}

impl<'a> SetChatAdministratorCustomTitle<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
        custom_title: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            custom_title: custom_title.into(),
        }
    }
}

impl SetChatAdministratorCustomTitle<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setChatAdministratorCustomTitle",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
