#![allow(deprecated)]

use is_macro::Is;

/// Chooses if some payment data is send to the provider.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[deprecated(
    since = "0.6.6",
    note = "use `SendInvoice::must_send_*_to_provider` methods which take a `bool`"
)]
#[must_use]
pub enum SendToProviderState {
    /// The data will be sent.
    Send,
    /// The data won't be sent.
    DoNotSend,
}

impl SendToProviderState {
    /// Checks if `self` is `Send`.
    #[must_use]
    pub fn should_send(self) -> bool {
        self == Self::Send
    }
}
