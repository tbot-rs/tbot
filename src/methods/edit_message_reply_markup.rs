use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        keyboard::inline,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Edits the inline keyboard of a message sent by the bot itself.
///
/// Reflects the [`editMessageReplyMarkup`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagereplymarkup
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageReplyMarkup<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    reply_markup: inline::Keyboard<'a>,
}

impl<'a> EditMessageReplyMarkup<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
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

impl EditMessageReplyMarkup<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
