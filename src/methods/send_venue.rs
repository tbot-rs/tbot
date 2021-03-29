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
use std::borrow::Cow;

/// Sends a venue.
///
/// Reflects the [`sendVenue`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvenue
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVenue<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    latitude: f64,
    longitude: f64,
    title: Cow<'a, str>,
    address: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendVenue<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        (latitude, longitude): (f64, f64),
        title: impl Into<Cow<'a, str>>,
        address: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    /// Configures the Foursquare ID of this venue.
    /// Reflects the `foursquare_id` parameter.
    pub fn foursquare_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.foursquare_id = Some(id.into());
        self
    }

    /// Configures the Foursquare type of this venue.
    /// Reflects the `foursquare_type` parameter.
    pub fn foursquare_type(mut self, fs_type: impl Into<Cow<'a, str>>) -> Self {
        self.foursquare_type = Some(fs_type.into());
        self
    }

    /// Configures the Google Places ID of this venue.
    /// Reflects the `google_place_id` parameter.
    pub fn google_place_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.google_place_id = Some(id.into());
        self
    }

    /// Configures the Google Places type of this venue.
    /// Reflects the `google_place_type` parameter.
    pub fn google_place_type(
        mut self,
        google_place_type: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.google_place_type = Some(google_place_type.into());
        self
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this venue is sent in reply to.
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

impl SendVenue<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "sendVenue",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
