use crate::types::parameters::Invoice as InvoiceParams;

use serde::Serialize;

/// Represents an [`InputInvoiceMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputinvoicemessagecontent
#[derive(Debug, Serialize, Eq, PartialEq, Clone, Hash)]
#[must_use]
pub struct Invoice {
    #[serde(flatten)]
    invoice: InvoiceParams,
}

impl Invoice {
    /// Construct an `Invoice`.
    pub const fn new(invoice: InvoiceParams) -> Self {
        Self { invoice }
    }
}
