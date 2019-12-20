/// Chooses if a piece of data is required.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum Requirement {
    /// The data is required.
    Required,
    /// The data is not required.
    NotRequired,
}

impl Requirement {
    /// Checks if `self` is `Required`.
    #[must_use]
    pub fn is_required(self) -> bool {
        self == Self::Required
    }
}
