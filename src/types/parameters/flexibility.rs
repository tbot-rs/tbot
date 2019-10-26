/// Chooses if price is flexible (i.e. it depends on some factor).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Flexibility {
    /// The price is flexible.
    Flexible,
    /// The price is not flexible.
    Inflexible,
}

impl Flexibility {
    /// Checks if `self` is `Flexible`.
    pub fn is_flexible(self) -> bool {
        self == Self::Flexible
    }

    /// Checks if `self` is `Inflexible`.
    pub fn is_inflexible(self) -> bool {
        self == Self::Inflexible
    }
}
