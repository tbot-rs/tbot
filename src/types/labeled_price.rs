use serde::Serialize;

/// Represents an [`LabeledPrice`].
///
/// [`LabeledPrice`]: https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
// todo: #[non_exhaustive]
pub struct LabeledPrice<'a> {
    label: &'a str,
    amount: u32,
}

impl<'a> LabeledPrice<'a> {
    /// Constructs a `LabeledPrice`.
    pub const fn new(label: &'a str, amount: u32) -> Self {
        Self {
            label,
            amount,
        }
    }
}
