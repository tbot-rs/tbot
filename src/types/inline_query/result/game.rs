use crate::types::value;
use serde::Serialize;

/// Represents an [`InlineQueryResultGame`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultgame
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Game<'a> {
    game_short_name: value::String<'a>,
}

impl<'a> Game<'a> {
    /// Constructs a `Game`.
    pub fn new(short_name: impl Into<value::String<'a>>) -> Self {
        Self {
            game_short_name: short_name.into(),
        }
    }
}
