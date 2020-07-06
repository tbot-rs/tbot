use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`InlineQueryResultGame`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultgame
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Game<'a> {
    game_short_name: Cow<'a, str>,
}

impl<'a> Game<'a> {
    /// Constructs a `Game`.
    pub fn new(game_short_name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            game_short_name: game_short_name.into(),
        }
    }
}
