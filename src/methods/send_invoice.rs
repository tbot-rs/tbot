use super::call_method;
#[allow(deprecated)]
use crate::{
    connectors::Client,
    errors, token,
    types::{
        chat,
        keyboard::inline,
        message::{self, Message},
        parameters::{
            Flexibility, NotificationState, Photo, Requirement,
            SendToProviderState,
        },
        LabeledPrice,
    },
};
use serde::Serialize;

/// Sends an invoice.
///
/// Reflects the [`sendInvoice`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendinvoice
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendInvoice<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
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
    need_phone_number: Option<bool>,
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

impl<'a> SendInvoice<'a> {
    #[allow(clippy::too_many_arguments)] // I know, brother
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
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
            need_phone_number: None,
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

    /// Configures data for your payment provider.
    /// Reflects the `provider_data` parameter.
    pub fn provider_data(mut self, provider_data: &'a str) -> Self {
        self.provider_data = Some(provider_data);
        self
    }

    /// Configures a photo for the invoice.
    /// Reflects the `photo_url`, `photo_width` and `photo_height` parameters.
    pub fn photo(mut self, photo: Photo<'a>) -> Self {
        self.photo = Some(photo);
        self
    }

    /// Configures if the user must specify their name.
    /// Reflects the `need_name` parameters.
    pub fn is_name_needed(mut self, is_needed: bool) -> Self {
        self.need_name = Some(is_needed);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_name_needed` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn name(self, is_needed: Requirement) -> Self {
        self.is_name_needed(is_needed.is_required())
    }

    /// Configures if the user must specify their phone number.
    /// Reflects the `need_phone_number` parameter.
    pub fn is_phone_number_needed(mut self, is_needed: bool) -> Self {
        self.need_phone_number = Some(is_needed);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_phone_number_needed` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn phone_number(self, is_needed: Requirement) -> Self {
        self.is_phone_number_needed(is_needed.is_required())
    }

    /// Configures if the user must specify their email.
    /// Reflects the `need_email` parameter.
    pub fn is_email_needed(mut self, is_needed: bool) -> Self {
        self.need_email = Some(is_needed);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_email_needed` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn email(self, is_needed: Requirement) -> Self {
        self.is_email_needed(is_needed.is_required())
    }

    /// Configures if the user must specify their shipping address.
    /// Reflects the `need_shipping_address` parameter.
    pub fn is_shipping_address_needed(mut self, is_needed: bool) -> Self {
        self.need_shipping_address = Some(is_needed);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_shipping_address_needed` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn shipping_address(self, is_needed: Requirement) -> Self {
        self.is_shipping_address_needed(is_needed.is_required())
    }

    /// Configures if the user's phone must be sent to your payment provider.
    /// Reflects the `send_phone_number_to_provider` parameter.
    pub fn should_send_phone_number_to_provider(
        mut self,
        should_send: bool,
    ) -> Self {
        self.send_phone_number_to_provider = Some(should_send);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `should_send_phone_number_to_provider` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn should_share_phone(self, should_send: SendToProviderState) -> Self {
        self.should_send_phone_number_to_provider(should_send.should_send())
    }

    /// Configures if the user's email must be sent to your payment provider.
    /// Reflects the `send_email_to_provider` parameter.
    pub fn should_send_email_to_provider(mut self, must_send: bool) -> Self {
        self.send_email_to_provider = Some(must_send);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `should_send_email_to_provider` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn should_share_email(self, should_send: SendToProviderState) -> Self {
        self.should_send_email_to_provider(should_send.should_send())
    }

    /// Configures if the final price depends on the shipping method.
    /// Reflects the `is_flexible` parameter.
    pub fn is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_flexible` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn flexibility(self, flexibility: Flexibility) -> Self {
        self.is_flexible(flexibility.is_flexible())
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_notification_disabled` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn notification(self, state: NotificationState) -> Self {
        self.is_notification_disabled(state.is_disabled())
    }

    /// Configures which message this invoice is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `in_reply_to`"
    )]
    pub fn reply_to_message_id(self, id: message::Id) -> Self {
        self.in_reply_to(id)
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl SendInvoice<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "sendInvoice",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
