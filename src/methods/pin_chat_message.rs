use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    token,
    types::{
        message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
};
use serde::Serialize;

/// Pins a message in a chat.
///
/// Reflects the [`pinChatMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#pinchatmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct PinChatMessage<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a, C> PinChatMessage<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Configures if the message will be pinned silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }
}

impl<C: Connector> PinChatMessage<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            self.token,
            "pinChatMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
