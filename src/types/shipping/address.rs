use serde::Deserialize;

/// Represents [`ShippingAddress`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#shippingaddress
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Address {
    /// The ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// The state, if apllicable.
    pub state: String,
    /// The city.
    pub city: String,
    /// The first line of the address.
    pub street_line1: String,
    /// The second line of the address.
    pub street_line2: String,
    /// The post code.
    pub post_code: String,
}
