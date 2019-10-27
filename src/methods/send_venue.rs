use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    Token,
};
use serde::Serialize;

/// Sends a venue.
///
/// Reflects the [`sendVenue`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendvenue
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendVenue<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    latitude: f64,
    longitude: f64,
    title: &'a str,
    address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendVenue<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        (latitude, longitude): (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures the Foursquare ID of this venue.
    /// Reflects the `foursquare_id` parameter.
    pub fn foursquare_id(mut self, id: &'a str) -> Self {
        self.foursquare_id = Some(id);
        self
    }

    /// Configures the Foursquare type of this venue.
    /// Reflects the `foursquare_type` parameter.
    pub fn foursquare_type(mut self, fs_type: &'a str) -> Self {
        self.foursquare_type = Some(fs_type);
        self
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures which message this venue is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
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

impl<C: Connector> SendVenue<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        send_method(
            self.client,
            &self.token,
            "sendVenue",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
