use serde::{
    de::{Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeMap, Serializer},
};
use std::fmt::{self, Formatter};

/// A placeholder currently holding no information. See [Bots API docs].
///
/// [Bots API docs]: https://core.telegram.org/bots/api#callbackgame
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Game;

impl Serialize for Game {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_map(Some(0))?.end()
    }
}

struct GameVisitor;

impl<'v> Visitor<'v> for GameVisitor {
    type Value = Game;

    fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "struct Game")
    }

    fn visit_map<V>(self, _map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        Ok(Game)
    }
}

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Game", &[], GameVisitor)
    }
}
