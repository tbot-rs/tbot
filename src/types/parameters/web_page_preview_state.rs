/// Chooses if a web page preview will be shown.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
#[must_use]
pub enum WebPagePreviewState {
    /// The preview will be enabled.
    Enabled,
    /// The preview will be disabled.
    Disabled,
}

impl WebPagePreviewState {
    /// Checks if the state is `Enabled`.
    #[must_use]
    pub fn is_enabled(self) -> bool {
        self == Self::Enabled
    }

    /// Checks if the state is `Disabled`.
    #[must_use]
    pub fn is_disabled(self) -> bool {
        self == Self::Disabled
    }
}
