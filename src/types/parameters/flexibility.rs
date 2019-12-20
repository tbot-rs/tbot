/// Chooses if price is flexible (i.e. it depends on some factor).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub enum Flexibility {
    /// The price is flexible.
    Flexible,
    /// The price is not flexible.
    Inflexible,
}

impl Flexibility {
    /// Checks if `self` is `Flexible`.
    #[must_use]
    pub fn is_flexible(self) -> bool {
        self == Self::Flexible
    }

    /// Checks if `self` is `Inflexible`.
    #[must_use]
    pub fn is_inflexible(self) -> bool {
        self == Self::Inflexible
    }
}
