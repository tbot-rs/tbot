use crate::types::{
    value::{self, Seq},
    LabeledPrice,
};
use serde::Serialize;

/// Represents a [`ShippingOption`][docs].
///
/// [docs]: https://core.telegram.org/bots/api/#shippingoption
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Option<'a> {
    id: value::String<'a>,
    title: value::String<'a>,
    prices: Seq<'a, LabeledPrice<'a>>,
}

impl<'a> Option<'a> {
    /// Constructs a shipping `Option`.
    pub fn new(
        id: impl Into<value::String<'a>>,
        title: impl Into<value::String<'a>>,
        prices: impl Into<Seq<'a, LabeledPrice<'a>>>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prices: prices.into(),
        }
    }
}
