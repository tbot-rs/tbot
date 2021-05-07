//! Types related to games.

use super::{message::Text, Animation, PhotoSize};
use serde::de::{self, Deserializer, IgnoredAny, MapAccess, Visitor};
use serde::Deserialize;

mod high_score;

pub use high_score::HighScore;

/// Represents a [`Game`].
///
/// [`Game`]: https://core.telegram.org/bots/api#game
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub struct Game {
    /// The title of the game.
    pub title: String,
    /// The description of the game.
    pub description: String,
    /// The photo of the game.
    pub photo: Vec<PhotoSize>,
    /// The text of the game.
    pub text: Option<Text>,
    /// The animation of the game.
    pub animation: Option<Animation>,
}

const TITLE: &str = "title";
const DESCRIPTION: &str = "description";
const PHOTO: &str = "photo";
const TEXT: &str = "text";
const TEXT_ENTITIES: &str = "text_entities";
const ANIMATION: &str = "animation";

struct GameVisitor;

impl<'v> Visitor<'v> for GameVisitor {
    type Value = Game;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct Game")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut title = None;
        let mut description = None;
        let mut photo = None;
        let mut text = None;
        let mut text_entities = None;
        let mut animation = None;

        while let Some(key) = map.next_key()? {
            match key {
                TITLE => title = Some(map.next_value()?),
                DESCRIPTION => description = Some(map.next_value()?),
                PHOTO => photo = Some(map.next_value()?),
                TEXT => text = Some(map.next_value()?),
                TEXT_ENTITIES => text_entities = Some(map.next_value()?),
                ANIMATION => animation = Some(map.next_value()?),
                _ => drop(map.next_value::<IgnoredAny>()),
            }
        }

        let text = text.map(|text| Text {
            value: text,
            entities: text_entities.unwrap_or_default(),
        });

        Ok(Game {
            title: title.ok_or_else(|| de::Error::missing_field(TITLE))?,
            description: description
                .ok_or_else(|| de::Error::missing_field(DESCRIPTION))?,
            photo: photo.ok_or_else(|| de::Error::missing_field(PHOTO))?,
            text,
            animation,
        })
    }
}

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Game",
            &[TITLE, DESCRIPTION, PHOTO, TEXT, TEXT_ENTITIES, ANIMATION],
            GameVisitor,
        )
    }
}
