use super::*;

/// Represents an [`Invoice`].
///
/// [`Invoice`]: https://core.telegram.org/bots/api#invoice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
// todo: #[non_exhaustive]
pub struct Invoice {
    /// The title of the invoice.
    pub title: String,
    /// The description of the invoice.
    pub description: String,
    /// The start parameter of the invoice.
    pub start_parameter: String,
    /// The currency of the invoice.
    pub currency: String,
    /// The total amount of the invoice.
    pub total_amount: u32,
}
