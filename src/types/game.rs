//! Types related to games.

use super::{message::text::Entity, Animation, PhotoSize};
use serde::Deserialize;

mod high_score;

pub use high_score::*;

/// Represents a [`Game`].
///
/// [`Game`]: https://core.telegram.org/bots/api#game
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Game {
    /// The title of the game.
    pub title: String,
    /// The description of the game.
    pub description: String,
    /// The photo of the game.
    pub photo: Vec<PhotoSize>,
    // todo: replace with `Option<message::Text>`
    /// The text of the game.
    pub text: Option<String>,
    /// The text entities of the game.
    pub text_entities: Option<Vec<Entity>>,
    /// The animation of the game.
    pub animation: Option<Animation>,
}
