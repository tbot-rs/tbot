use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
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
    chat_id: ChatId<'a>,
    from_chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a> ForwardMessage<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        from_chat_id: impl ImplicitChatId<'a>,
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

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
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
