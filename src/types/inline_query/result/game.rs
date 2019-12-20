use serde::Serialize;

/// Represents an [`InlineQueryResultGame`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultgame
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Game<'a> {
    game_short_name: &'a str,
}

impl<'a> Game<'a> {
    /// Constructs a `Game`.
    pub const fn new(game_short_name: &'a str) -> Self {
        Self { game_short_name }
    }
}
