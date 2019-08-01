use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        keyboard, message,
        parameters::{ChatId, ImplicitChatId, NotificationState},
        value::{self, Ref},
    },
};

/// Represents the [`sendVenue`][docs] method.
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
    title: value::String<'a>,
    address: value::String<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<value::String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<Ref<'a, keyboard::Any<'a>>>,
}

impl<'a, C> SendVenue<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        (latitude, longitude): (f64, f64),
        title: impl Into<value::String<'a>>,
        address: impl Into<value::String<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `foursquare_id`.
    pub fn foursquare_id(mut self, id: impl Into<value::String<'a>>) -> Self {
        self.foursquare_id = Some(id.into());
        self
    }

    /// Configures `foursquare_type`.
    pub fn foursquare_type(
        mut self,
        fs_type: impl Into<value::String<'a>>,
    ) -> Self {
        self.foursquare_type = Some(fs_type.into());
        self
    }

    /// Configures `disable_notification`.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<Ref<'a, keyboard::Any<'a>>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C> IntoFuture for SendVenue<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "sendVenue",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
