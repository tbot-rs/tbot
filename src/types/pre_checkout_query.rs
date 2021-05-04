//! Types related to pre-checkout queries.

use super::{OrderInfo, User};
use serde::Deserialize;

mod id;

pub use id::Id;

/// Represents [`PreCheckoutQuery`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#precheckoutquery
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct PreCheckoutQuery {
    /// The ID of the query.
    pub id: Id,
    /// The user who sent the query.
    pub from: User,
    /// The currency of of the invoice.
    pub currency: String,
    /// The total price.
    pub total_amount: u32,
    /// The invoice payload sent previously by the bot.
    pub invoice_payload: String,
    /// The ID of the chosen shipping option.
    pub shipping_option_id: Option<String>,
    /// The order information.
    pub order_info: Option<OrderInfo>,
}
