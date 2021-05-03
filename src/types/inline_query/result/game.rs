use serde::Serialize;

/// Represents an [`InlineQueryResultGame`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultgame
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Game {
    game_short_name: String,
}

impl Game {
    /// Constructs a `Game`.
    pub fn new(game_short_name: impl Into<String>) -> Self {
        Self {
            game_short_name: game_short_name.into(),
        }
    }
}
