use crate::types::value;
use serde::Serialize;

/// Represents an [`LabeledPrice`].
///
/// [`LabeledPrice`]: https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
// todo: #[non_exhaustive]
pub struct LabeledPrice<'a> {
    label: value::String<'a>,
    amount: u32,
}

impl<'a> LabeledPrice<'a> {
    /// Constructs a `LabeledPrice`.
    pub fn new(label: impl Into<value::String<'a>>, amount: u32) -> Self {
        Self {
            label: label.into(),
            amount,
        }
    }
}
