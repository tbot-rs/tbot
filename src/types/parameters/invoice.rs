//! Types related to invoice parameters.

use crate::types::{
    parameters::{Photo, Tip},
    LabeledPrice,
};
use serde::Serialize;

/// Miscellaneous parameters for Invoice methods and types.
#[derive(Debug, Serialize, Eq, PartialEq, Clone, Hash)]
#[must_use]
pub struct Invoice {
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
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
    #[serde(flatten)]
    tip: Option<Tip>,
}

impl Invoice {
    /// Constructs an `Invoice`.
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        provider_token: impl Into<String>,
        currency: impl Into<String>,
        prices: impl Into<Vec<LabeledPrice>>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices: prices.into(),
            provider_data: None,
            photo: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
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

    /// Configures tip for the invoice.
    /// Reflects the `tip` parameter.
    #[allow(clippy::missing_const_for_fn)]
    pub fn tip(mut self, tip: Tip) -> Self {
        self.tip = Some(tip);
        self
    }
}
