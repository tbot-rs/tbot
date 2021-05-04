use crate::types::LabeledPrice;
use serde::Serialize;

/// Represents a [`ShippingOption`][docs].
///
/// [docs]: https://core.telegram.org/bots/api/#shippingoption
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Option {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

impl Option {
    /// Constructs a shipping `Option`.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        prices: impl Into<Vec<LabeledPrice>>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prices: prices.into(),
        }
    }
}
