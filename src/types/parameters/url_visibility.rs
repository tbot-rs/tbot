/// Represent URL visibility state.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
#[must_use]
pub enum UrlVisibility {
    /// The URL is visible.
    Shown,
    /// The URL is hidden.
    Hidden,
}

impl UrlVisibility {
    /// Checks if the visibility is set to `Shown`.
    #[must_use]
    pub fn is_shown(self) -> bool {
        self == Self::Shown
    }

    /// Checks if the visibility is set to `Hidden`.
    #[must_use]
    pub fn is_hidden(self) -> bool {
        self == Self::Hidden
    }
}
