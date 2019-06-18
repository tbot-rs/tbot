use crate::types::LabeledPrice;
use serde::Serialize;

/// Represents a [`ShippingOption`][docs].
///
/// [docs]: https://core.telegram.org/bots/api/#shippingoption
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Option<'a> {
    id: &'a str,
    title: &'a str,
    prices: &'a [LabeledPrice<'a>],
}

impl<'a> Option<'a> {
    /// Constructs a shipping `Option`.
    pub fn new(
        id: &'a str,
        title: &'a str,
        prices: &'a [LabeledPrice<'a>],
    ) -> Self {
        Self {
            id,
            title,
            prices,
        }
    }
}
