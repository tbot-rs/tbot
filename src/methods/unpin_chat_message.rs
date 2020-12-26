use super::call_method;
use crate::types::{message::Id};
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Unpins a chat message.
///
/// Reflects the [`unpinChatMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#unpinchatmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UnpinChatMessage<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    message_id: Option<Id>,
}

impl<'a> UnpinChatMessage<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id: None,
        }
    }

    /// Configures which message to unpin.
    /// Reflects `message_id` parameter.
    pub const fn message_id(mut self, message_id: Id) -> Self {
        self.message_id = Some(message_id);
        self
    }
}

impl UnpinChatMessage<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "unpinChatMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
