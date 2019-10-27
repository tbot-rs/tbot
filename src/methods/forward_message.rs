use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    token,
};
use serde::Serialize;

/// Forwards a message.
///
/// Reflects the [`forwardMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#forwardmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct ForwardMessage<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    from_chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a, C> ForwardMessage<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        from_chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> Self {
        Self {
            client,
            token,
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

impl<C: Connector> ForwardMessage<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "forwardMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
