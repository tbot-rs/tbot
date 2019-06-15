use serde::ser::{Serialize, SerializeMap, Serializer};

/// A placeholder currently holding no information. See [Bots API docs].
///
/// [Bots API docs]: https://core.telegram.org/bots/api#callbackgame
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct CallbackGame;

impl Serialize for CallbackGame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_map(Some(0))?.end()
    }
}
