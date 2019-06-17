/// Chooses if a piece of data is required.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Requirement {
    /// The data is required.
    Required,
    /// The data is not required.
    NotRequired,
}

impl Requirement {
    /// Checks if `self` is `Required`.
    pub fn is_required(self) -> bool {
        self == Requirement::Required
    }
}
