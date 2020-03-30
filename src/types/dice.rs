use serde::Deserialize;

/// Represents a [`Dice`].
///
/// [`Dice`]: https://core.telegram.org/bots/api#dice
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
pub struct Dice {
    /// The value of the dice in the range [1, 6].
    pub value: u8,
}
