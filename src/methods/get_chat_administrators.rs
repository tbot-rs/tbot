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

/// Gets information about a chat's admins.
///
/// Reflects the [`getChatAdministrators`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatadministrators
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatAdministrators<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
}

impl<'a> GetChatAdministrators<'a> {
    pub(crate) fn new(bot: &'a InnerBot, chat_id: impl ImplicitChatId) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl GetChatAdministrators<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<chat::Member>, errors::MethodCall> {
        call_method(
            self.bot,
            "getChatAdministrators",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
