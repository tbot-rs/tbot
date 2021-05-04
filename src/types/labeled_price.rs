use serde::Serialize;

/// Represents an [`LabeledPrice`].
///
/// [`LabeledPrice`]: https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[non_exhaustive]
#[must_use]
pub struct LabeledPrice {
    label: String,
    amount: u32,
}

impl LabeledPrice {
    /// Constructs a `LabeledPrice`.
    pub fn new(label: impl Into<String>, amount: u32) -> Self {
        Self {
            label: label.into(),
            amount,
        }
    }
}
