use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard::inline,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, ParseMode, Text},
    },
};
use serde::Serialize;

/// Edits the caption of a media message sent by the bot itself.
///
/// Reflects the [`editMessageCaption`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagecaption
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageCaption<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    message_id: message::Id,
    caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditMessageCaption<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        caption: impl Into<Text>,
    ) -> Self {
        let caption = caption.into();

        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            caption: caption.text,
            parse_mode: caption.parse_mode,
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditMessageCaption<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "editMessageCaption",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
