use serde::{Deserialize, Serialize};

/// Represents a [`BotCommand`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#botcommand
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[must_use]
pub struct BotCommand {
    /// The command's text.
    pub command: String,
    /// The command's decription.
    pub description: String,
}

impl BotCommand {
    /// Constructs a new `BotCommand`.
    pub fn new(
        command: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
        }
    }
}
