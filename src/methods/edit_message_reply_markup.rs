use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        keyboard::inline,
        message,
        parameters::{ChatId, ImplicitChatId},
    },
};

/// Edits the inline keyboard of a message sent by the bot itself.
///
/// Reflects the [`editMessageReplyMarkup`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageReplyMarkup<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    reply_markup: inline::Keyboard<'a>,
}

impl<'a, C> EditMessageReplyMarkup<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        reply_markup: inline::Keyboard<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            reply_markup,
        }
    }
}

impl<C: Connector> EditMessageReplyMarkup<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<types::Message, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
