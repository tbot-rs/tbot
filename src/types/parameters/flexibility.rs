use is_macro::Is;

/// Chooses if price is flexible (i.e. it depends on some factor).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Is)]
#[must_use]
pub enum Flexibility {
    /// The price is flexible.
    Flexible,
    /// The price is not flexible.
    Inflexible,
}
