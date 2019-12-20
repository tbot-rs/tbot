use super::shipping;
use serde::Deserialize;

/// Represents [`OrderInfo`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#orderinfo
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
#[must_use]
pub struct OrderInfo {
    /// User's name.
    pub name: Option<String>,
    /// User's phone number.
    pub phone_number: Option<String>,
    /// User's email.
    pub email: Option<String>,
    /// User's shipping address.
    pub shipping_address: Option<shipping::Address>,
}
