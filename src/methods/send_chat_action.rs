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

/// Sends a chat action.
///
/// Reflects the [`sendChatAction`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendchataction
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendChatAction<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    action: chat::Action,
}

impl<'a> SendChatAction<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        action: chat::Action,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            action,
        }
    }
}

impl SendChatAction<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "sendChatAction",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
