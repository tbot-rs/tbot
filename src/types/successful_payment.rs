use super::OrderInfo;
use serde::Deserialize;

/// Represents a [`SuccessfulPayment`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#successfulpayment
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
#[must_use]
pub struct SuccessfulPayment {
    /// Currency of the payment.
    pub currency: String,
    /// The total price.
    pub total_amount: u32,
    /// The payload previously specified by the bot.
    pub invoice_payload: String,
    /// The ID of the chosen shipping option.
    pub shipping_option_id: Option<String>,
    /// The information about the order.
    pub order_info: Option<OrderInfo>,
    /// Telegram payment ID.
    pub telegram_payment_charge_id: String,
    /// Provider payment ID.
    pub provider_payment_charge_id: String,
}
