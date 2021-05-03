use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
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
    bot: &'a InnerBot,
    chat_id: ChatId,
    message_id: message::Id,
    reply_markup: inline::Keyboard<'a>,
}

impl<'a> EditMessageReplyMarkup<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        reply_markup: inline::Keyboard<'a>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            reply_markup,
        }
    }
}

impl EditMessageReplyMarkup<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "editMessageReplyMarkup",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
