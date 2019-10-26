/// Chooses if some payment data is send to the provider.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SendToProviderState {
    /// The data will be sent.
    Send,
    /// The data won't be sent.
    DoNotSend,
}

impl SendToProviderState {
    /// Checks if `self` is `Send`.
    pub fn should_send(self) -> bool {
        self == Self::Send
    }
}
