use super::*;
use crate::{
    internal::{BoxFuture, Client},
    types::{
        chat,
        keyboard::inline,
        message,
        parameters::{
            Flexibility, NotificationState, Photo, Requirement,
            SendToProviderState,
        },
        LabeledPrice,
    },
};
use serde::Serialize;

/// Represents the [`sendInvoice`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendinvoice
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendInvoice<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: chat::Id,
    title: &'a str,
    description: &'a str,
    payload: &'a str,
    provider_token: &'a str,
    start_parameter: &'a str,
    currency: &'a str,
    prices: &'a [LabeledPrice<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    photo: Option<Photo<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_photo_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> SendInvoice<'a, C> {
    #[allow(clippy::too_many_arguments)] // I know, brother
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<chat::Id>,
        title: &'a str,
        description: &'a str,
        payload: &'a str,
        provider_token: &'a str,
        start_parameter: &'a str,
        currency: &'a str,
        prices: &'a [LabeledPrice<'a>],
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
            provider_data: None,
            photo: None,
            need_name: None,
            need_photo_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `provider_data`.
    pub fn provider_data(mut self, provider_data: &'a str) -> Self {
        self.provider_data = Some(provider_data);
        self
    }

    /// Configures `photo_url`, `photo_width` and `photo_height`.
    pub fn photo(mut self, photo: Photo<'a>) -> Self {
        self.photo = Some(photo);
        self
    }

    /// Configures `need_name`.
    pub fn name(mut self, is_needed: Requirement) -> Self {
        self.need_name = Some(is_needed.is_required());
        self
    }

    /// Configures `need_photo_number`.
    pub fn photo_number(mut self, is_needed: Requirement) -> Self {
        self.need_photo_number = Some(is_needed.is_required());
        self
    }

    /// Configures `need_email`.
    pub fn email(mut self, is_needed: Requirement) -> Self {
        self.need_email = Some(is_needed.is_required());
        self
    }

    /// Configures `need_shipping_address`.
    pub fn shipping_address(mut self, is_needed: Requirement) -> Self {
        self.need_shipping_address = Some(is_needed.is_required());
        self
    }

    /// Configures `send_phone_number_to_provider`.
    pub fn should_share_phone(
        mut self,
        should_send: SendToProviderState,
    ) -> Self {
        self.send_phone_number_to_provider = Some(should_send.should_send());
        self
    }

    /// Configures `send_email_to_provider`.
    pub fn should_share_email(
        mut self,
        should_send: SendToProviderState,
    ) -> Self {
        self.send_email_to_provider = Some(should_send.should_send());
        self
    }

    /// Configures `is_flexible`.
    pub fn flexibility(mut self, flexibility: Flexibility) -> Self {
        self.is_flexible = Some(flexibility.is_flexible());
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
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<C> IntoFuture for SendInvoice<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "sendInvoice",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
