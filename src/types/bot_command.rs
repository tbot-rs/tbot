use serde::Deserialize;

/// Represents a [`BotCommand`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#botcommand
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct BotCommand {
    /// The command's text.
    pub command: String,
    /// The command's decription.
    pub description: String,
}
