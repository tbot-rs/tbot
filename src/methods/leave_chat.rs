use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Leaves a chat.
///
/// Reflects the [`leaveChat`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#leavechat
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct LeaveChat<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
}

impl<'a> LeaveChat<'a> {
    pub(crate) fn new(bot: &'a InnerBot, chat_id: impl ImplicitChatId) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl LeaveChat<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "leaveChat",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
