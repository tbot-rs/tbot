use serde::Serialize;

/// Represents a [`BotCommand`][docs] to be sent in the list of the bot's commands.
///
/// [docs]: https://core.telegram.org/bots/api#botcommand
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[non_exhaustive]
pub struct BotCommand {
    /// The command's text.
    command: String,
    /// The command's description.
    description: String,
}

impl BotCommand {
    /// Constructs a new `BotCommand`.
    #[must_use]
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
