use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        dice::Kind,
        keyboard, message,
        parameters::{ChatId, ImplicitChatId},
        Message,
    },
};
use serde::Serialize;

/// Sends a dice.
///
/// Reflects the [`sendDice`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#senddice
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendDice<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    #[serde(rename = "emoji")]
    kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendDice<'a> {
    pub(crate) fn new(bot: &'a InnerBot, chat_id: impl ImplicitChatId) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            kind: Kind::Dice,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    /// Сonfigures the dice's kind. Reflects the `emoji` parameter.
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub fn kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this dice is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub const fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures whether this message should be sent even
    /// if the replied-to message is not found.
    /// Reflects the `allow_sending_without_reply` parameter.
    pub const fn allow_sending_without_reply(mut self) -> Self {
        self.allow_sending_without_reply = true;
        self
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl SendDice<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "sendDice",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
