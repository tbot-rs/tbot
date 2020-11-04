use is_macro::Is;

/// Chooses if a web page preview will be shown.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[non_exhaustive]
#[must_use]
pub enum WebPagePreviewState {
    /// The preview will be enabled.
    Enabled,
    /// The preview will be disabled.
    Disabled,
}
