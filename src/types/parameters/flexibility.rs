#![allow(deprecated)]

use is_macro::Is;

/// Chooses if price is flexible (i.e. it depends on some factor).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[deprecated(
    since = "0.6.6",
    note = "Use `SendInvoice::is_flexible` which takes a `bool`"
)]
#[must_use]
pub enum Flexibility {
    /// The price is flexible.
    Flexible,
    /// The price is not flexible.
    Inflexible,
}
