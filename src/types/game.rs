use super::*;

/// Represents a [`Game`].
///
/// [`Game`]: https://core.telegram.org/bots/api#game
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Game {
    /// The title of the game.
    pub title: String,
    /// The description of the game.
    pub description: String,
    /// The photo of the game.
    pub photo: Vec<PhotoSize>,
    /// The text of the game.
    pub text: Option<String>,
    /// The text entities of the game.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// The animation of the game.
    pub animation: Option<Animation>,
}
