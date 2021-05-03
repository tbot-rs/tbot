use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Forwards a message.
///
/// Reflects the [`forwardMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#forwardmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct ForwardMessage<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a> ForwardMessage<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        from_chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }
}

impl ForwardMessage<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "forwardMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
