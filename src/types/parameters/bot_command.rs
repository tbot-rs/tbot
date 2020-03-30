use serde::Serialize;

/// Represents a [`BotCommand`][docs] to be sent in the list of the bot's commands.
///
/// [docs]: https://core.telegram.org/bots/api#botcommand
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[non_exhaustive]
pub struct BotCommand<'a> {
    /// The command's text.
    command: &'a str,
    /// The command's decription.
    description: &'a str,
}

impl<'a> BotCommand<'a> {
    /// Construсts a new `BotCommand`.
    #[must_use]
    pub const fn new(command: &'a str, description: &'a str) -> Self {
        Self {
            command,
            description,
        }
    }
}
