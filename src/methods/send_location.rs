use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Sends a location.
///
/// Reflects the [`sendLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendlocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendLocation<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendLocation<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    /// Confgiures for how long this location will be live and may be edited.
    /// Reflects the `live_period` parameter.
    pub const fn live_period(mut self, duration: u32) -> Self {
        self.live_period = Some(duration);
        self
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this location is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub const fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures whether this message should be sent even
    /// if the replied-to message is not found.
    /// Reflects the `allow_sending_without_reply` parameter.
    pub const fn allow_sending_without_reply(mut self) -> Self {
        self.allow_sending_without_reply = Some(true);
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

impl SendLocation<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "sendLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
