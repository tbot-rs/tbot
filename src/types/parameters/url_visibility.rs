use is_macro::Is;

/// Represent URL visibility state.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[non_exhaustive]
#[must_use]
pub enum UrlVisibility {
    /// The URL is visible.
    Shown,
    /// The URL is hidden.
    Hidden,
}
