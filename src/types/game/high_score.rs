use crate::types::User;
use serde::Deserialize;

/// Represents a [`GameHighScore`].
///
/// [`GameHighScore`]: https://core.telegram.org/bots/api#gamehighscore
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct HighScore {
    /// Position of the user in the high score table.
    pub position: u32,
    /// Information about the user.
    pub user: User,
    /// The user's score.
    pub score: i32,
}
