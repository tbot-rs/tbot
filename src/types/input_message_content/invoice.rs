use crate::types::parameters::Invoice as InvoiceParams;

use serde::Serialize;

/// Represents an [`InputInvoiceMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputinvoicemessagecontent
#[derive(Debug, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Invoice {
    #[serde(flatten)]
    invoice: InvoiceParams,
}
