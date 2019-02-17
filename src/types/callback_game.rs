use super::*;

/// A placeholder currently holding no information. See [Bots API docs].
///
/// [Bots API docs]: https://core.telegram.org/bots/api#callbackgame
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, Serialize)]
pub struct CallbackGame;
