use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard, location,
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
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<f64>,
    #[serde(flatten)]
    live_location: Option<location::Live>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any>,
}

impl<'a> SendLocation<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_location: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    /// Configures the radius of uncertainty for the location in meters, in range `1.0..=1500.0`.
    ///
    /// # Panics
    ///
    /// Panics if `horizontal_accuracy` is not in range `1.0..=1500.0`.
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: f64) -> Self {
        assert!(
            (1.0..=1500.0).contains(&horizontal_accuracy),
            "[tbot] Received invalid `horizontal_accuracy` in \
             `SendLocation::horizontal_accuracy`: \
             {}, must be in range `1.0..=1500.0`",
            horizontal_accuracy,
        );

        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    /// Confgiures a live location.
    pub const fn live_location(
        mut self,
        live_location: location::Live,
    ) -> Self {
        self.live_location = Some(live_location);
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
        self.allow_sending_without_reply = true;
        self
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(mut self, markup: impl Into<keyboard::Any>) -> Self {
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
