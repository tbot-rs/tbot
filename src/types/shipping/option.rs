use crate::types::{InteriorBorrow, LabeledPrice};
use serde::Serialize;
use std::borrow::Cow;

/// Represents a [`ShippingOption`][docs].
///
/// [docs]: https://core.telegram.org/bots/api/#shippingoption
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Option<'a> {
    id: Cow<'a, str>,
    title: Cow<'a, str>,
    prices: Cow<'a, [LabeledPrice<'a>]>,
}

impl<'a> Option<'a> {
    /// Constructs a shipping `Option`.
    pub fn new(
        id: impl Into<Cow<'a, str>>,
        title: impl Into<Cow<'a, str>>,
        prices: impl Into<Cow<'a, [LabeledPrice<'a>]>>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prices: prices.into(),
        }
    }
}

impl<'a> InteriorBorrow<'a> for Option<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            id: self.id.borrow_inside(),
            title: self.title.borrow_inside(),
            prices: self.prices.borrow_inside(),
        }
    }
}
