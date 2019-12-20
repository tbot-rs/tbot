//! Types related to shipping queries.

use crate::types::{shipping::Address, User};
use serde::Deserialize;
pub mod id;

pub use id::Id;

/// Represents a [`ShippingQuery`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#shippingquery
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
#[must_use]
pub struct Query {
    /// The ID of the query.
    pub id: Id,
    /// The user who sent the query.
    pub from: User,
    /// The invoice payload sent previously by the bot.
    pub invoice_payload: String,
    /// The shipping address specified by the user.
    pub shipping_address: Address,
}
