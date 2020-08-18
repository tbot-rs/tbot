use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        message,
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Deletes a message from a chat.
///
/// Reflects the [`deleteMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#deletemessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteMessage<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    message_id: message::Id,
}

impl<'a> DeleteMessage<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl DeleteMessage<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "deleteMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
