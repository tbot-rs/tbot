use is_macro::Is;

/// Chooses if a piece of data is required.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[must_use]
pub enum Requirement {
    /// The data is required.
    Required,
    /// The data is not required.
    NotRequired,
}
