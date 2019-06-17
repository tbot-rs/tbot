/// Chooses if a web page preview will be shown.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// todo: #[non_exhaustive]
pub enum WebPagePreviewState {
    /// The preview will be enabled.
    Enabled,
    /// The preview will be disabled.
    Disabled,
}

impl WebPagePreviewState {
    /// Checks if the state is `Enabled`.
    pub fn is_enabled(self) -> bool {
        self == WebPagePreviewState::Enabled
    }

    /// Checks if the state is `Disabled`.
    pub fn is_disabled(self) -> bool {
        self == WebPagePreviewState::Disabled
    }
}
