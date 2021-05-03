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
use std::{
    convert::TryInto,
    num::{NonZeroU16, NonZeroU32},
};

/// Edits a live location sent by the bot itself.
///
/// Reflects the [`editMessageLiveLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagelivelocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageLocation<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<NonZeroU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<NonZeroU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditMessageLocation<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
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
             `EditMessageLocation::horizontal_accuracy`: \
             {}, must be in range `1.0..=1500.0`",
            horizontal_accuracy,
        );

        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    /// Configures the direction in which the user is headed. The value must be
    /// in range `1..=360`. Reflects the `heading` parameter.
    ///
    /// # Panics
    ///
    /// Panics if `heading` is not in range `1..=360`.
    pub fn heading(mut self, heading: u16) -> Self {
        assert!(
            (1..=360).contains(&heading),
            "[tbot] Received invalid `heading` in `LiveLocation::heading`: \
             {}, must be in range `1..=360`",
            heading,
        );

        self.heading = Some(heading.try_into().unwrap());
        self
    }

    /// Configures the maximum distance for proximity alerts about a user
    /// approaching another, in meters. The value must be in range
    /// `1..=100_000`. Reflects the `proximity_alert_radius` paramter.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in range `1..=100_000`.
    pub fn proximity_alert_radius(mut self, radius: u32) -> Self {
        assert!(
            (1..=100_000).contains(&radius),
            "[tbot] Received invalid `radius` in \
             `LiveLocation::proximity_alert_radius`: {}, must be in range \
             `1..=100_000`",
            radius,
        );

        self.proximity_alert_radius = Some(radius.try_into().unwrap());
        self
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditMessageLocation<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "editMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
