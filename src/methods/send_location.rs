use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
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
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
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
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendLocation<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Confgiures for how long this location will be live and may be edited.
    /// Reflects the `live_period` parameter.
    pub fn live_period(mut self, duration: u32) -> Self {
        self.live_period = Some(duration);
        self
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures which message this location is sent in reply to.
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

impl SendLocation<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "sendLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
