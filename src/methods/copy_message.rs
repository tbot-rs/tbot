use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, ParseMode, Text},
    },
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// Reflects the [`MessageId`][docs] type.
//
// [docs]: https://core.telegram.org/bots/api#messageid
#[derive(Deserialize, Debug, Clone)]
struct MessageId {
    message_id: message::Id,
}

/// Copies a message.
///
/// Reflects the [`copyMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#copymessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct CopyMessage<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> CopyMessage<'a> {
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
            caption: None,
            parse_mode: None,
            disable_notification: None,
            allow_sending_without_reply: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Replaces the original caption with the provided one.
    /// Reflects the `caption` and `parse_mode` parameters.
    #[allow(clippy::missing_const_for_fn)]
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures whether this message should be sent even
    /// if the replied-to message is not found.
    /// Reflects the `allow_sending_without_reply` parameter.
    pub const fn allow_sending_without_reply(mut self) -> Self {
        self.allow_sending_without_reply = true;
        self
    }

    /// Configures which message this message is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub const fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    // Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl CopyMessage<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<message::Id, errors::MethodCall> {
        let result = call_method::<MessageId>(
            self.bot,
            "copyMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(result.message_id)
    }
}
