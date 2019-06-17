/// Represent URL visibility state.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// todo: #[non_exhaustive]
pub enum UrlVisibility {
    /// The URL is visible.
    Shown,
    /// The URL is hidden.
    Hidden,
}

impl UrlVisibility {
    /// Checks if the visibility is set to `Shown`.
    pub fn is_shown(self) -> bool {
        self == UrlVisibility::Shown
    }

    /// Checks if the visibility is set to `Hidden`.
    pub fn is_hidden(self) -> bool {
        self == UrlVisibility::Hidden
    }
}
