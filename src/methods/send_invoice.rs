use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        chat,
        keyboard::inline,
        message::{self, Message},
        parameters::Photo,
        LabeledPrice,
    },
};
use serde::Serialize;

/// Represents tip parameters.
#[derive(Debug, Clone, Serialize)]
pub struct Tip {
    max_tip_amount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<u32>>,
}

impl Tip {
    pub fn with_max(max_tip: u32) -> Self {
        Self {
            max_tip_amount: max_tip,
            suggested_tip_amounts: None,
        }
    }

    /// Configures suggested tip amounts for the invoice.
    /// At most 4 suggestions can be specified.
    /// Reflects the `suggested_tip_amounts` parameter.
    ///
    /// # Panics
    ///
    /// Panics if there are more than 4 elements.
    pub fn suggested_tips(mut self, suggested: impl Into<Vec<u32>>) -> Self {
        let mut suggested = suggested.into();
        assert!(
            (1..=4).contains(&suggested.len()),
            "[tbot] Received invalid `suggested` in \
             `Tip::suggested_tips` must have at most 4 elements.",
        );
        suggested.sort();
        self.suggested_tip_amounts = Some(suggested);
        self
    }
}

/// Sends an invoice.
///
/// Reflects the [`sendInvoice`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendinvoice
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendInvoice<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: chat::Id,
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    photo: Option<Photo>,
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
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    tip: Option<Tip>,
}

impl<'a> SendInvoice<'a> {
    #[allow(clippy::too_many_arguments)] // I know, brother
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl Into<chat::Id>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        provider_token: impl Into<String>,
        currency: impl Into<String>,
        prices: impl Into<Vec<LabeledPrice>>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices: prices.into(),
            start_parameter: None,
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
            allow_sending_without_reply: false,
            reply_markup: None,
            tip: None,
        }
    }

    /// Configures data for your payment provider.
    /// Reflects the `provider_data` parameter.
    pub fn provider_data(mut self, provider_data: impl Into<String>) -> Self {
        self.provider_data = Some(provider_data.into());
        self
    }

    /// Configures a photo for the invoice.
    /// Reflects the `photo_url`, `photo_width` and `photo_height` parameters.
    #[allow(clippy::missing_const_for_fn)]
    pub fn photo(mut self, photo: Photo) -> Self {
        self.photo = Some(photo);
        self
    }

    /// Configures whether the user must specify their name.
    /// Reflects the `need_name` parameters.
    pub const fn is_name_needed(mut self, is_needed: bool) -> Self {
        self.need_name = Some(is_needed);
        self
    }

    /// Configures whether the user must specify their phone number.
    /// Reflects the `need_phone_number` parameter.
    pub const fn is_phone_number_needed(mut self, is_needed: bool) -> Self {
        self.need_phone_number = Some(is_needed);
        self
    }

    /// Configures whether the user must specify their email.
    /// Reflects the `need_email` parameter.
    pub const fn is_email_needed(mut self, is_needed: bool) -> Self {
        self.need_email = Some(is_needed);
        self
    }

    /// Configures whether the user must specify their shipping address.
    /// Reflects the `need_shipping_address` parameter.
    pub const fn is_shipping_address_needed(mut self, is_needed: bool) -> Self {
        self.need_shipping_address = Some(is_needed);
        self
    }

    /// Configures whether the user's phone must be sent to your payment
    /// provider. Reflects the `send_phone_number_to_provider` parameter.
    pub const fn should_send_phone_number_to_provider(
        mut self,
        must_send: bool,
    ) -> Self {
        self.send_phone_number_to_provider = Some(must_send);
        self
    }

    /// Configures whether the user's email must be sent to your payment
    /// provider. Reflects the `send_email_to_provider` parameter.
    pub const fn should_send_email_to_provider(
        mut self,
        must_send: bool,
    ) -> Self {
        self.send_email_to_provider = Some(must_send);
        self
    }

    /// Configures whether the final price depends on the shipping method.
    /// Reflects the `is_flexible` parameter.
    pub const fn is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this invoice is sent in reply to.
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
    #[allow(clippy::missing_const_for_fn)]
    pub fn reply_markup(mut self, markup: inline::Keyboard) -> Self {
        self.reply_markup = Some(markup);
        self
    }

    /// Configures unique deep-linking parameter for "Pay button" to redirect.
    /// Reflects the `start_parameter` parameter.
    pub fn start_parameter(
        mut self,
        start_parameter: impl Into<String>,
    ) -> Self {
        self.start_parameter = Some(start_parameter.into());
        self
    }

    /// Configures tip for the invoice.
    /// Reflects the `tip` field (`max_tip_amount`, `suggested_tip_amounts`).
    pub fn tip(mut self, tip: Tip) -> Self {
        self.tip = Some(tip);
        self
    }
}

impl SendInvoice<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "sendInvoice",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
