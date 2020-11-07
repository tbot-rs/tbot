use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, ParseMode, Text},
    },
};
use serde::Serialize;
use std::borrow::Cow;

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
    chat_id: ChatId<'a>,
    from_chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> CopyMessage<'a> {
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
            caption: None,
            parse_mode: None,
            disable_notification: None,
            allow_sending_without_reply: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Replaces the original caption with the provided one.
    /// Reflects the `caption` and `parse_mode` parameters.
    #[allow(clippy::missing_const_for_fn)]
    pub fn caption(mut self, caption: Text<'a>) -> Self {
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

    /// If `true`, then you may reply to already-deleted messages.
    /// Refelcts the `allow_sending_without_reply` parameter.
    pub const fn is_sending_without_reply_allowed(
        mut self,
        is_allowed: bool,
    ) -> Self {
        self.allow_sending_without_reply = Some(is_allowed);
        self
    }

    /// Configures which message this document is sent in reply to.
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
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "copyMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
