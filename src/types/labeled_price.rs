use crate::types::InteriorBorrow;
use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`LabeledPrice`].
///
/// [`LabeledPrice`]: https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[non_exhaustive]
#[must_use]
pub struct LabeledPrice<'a> {
    label: Cow<'a, str>,
    amount: u32,
}

impl<'a> LabeledPrice<'a> {
    /// Constructs a `LabeledPrice`.
    pub fn new(label: impl Into<Cow<'a, str>>, amount: u32) -> Self {
        Self {
            label: label.into(),
            amount,
        }
    }
}

impl<'a> InteriorBorrow<'a> for LabeledPrice<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            label: self.label.borrow_inside(),
            ..*self
        }
    }
}
