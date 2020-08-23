use crate::types::InteriorBorrow;
use serde::Serialize;
use std::borrow::Cow;

/// Represents a [`BotCommand`][docs] to be sent in the list of the bot's commands.
///
/// [docs]: https://core.telegram.org/bots/api#botcommand
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[non_exhaustive]
pub struct BotCommand<'a> {
    /// The command's text.
    command: Cow<'a, str>,
    /// The command's description.
    description: Cow<'a, str>,
}

impl<'a> BotCommand<'a> {
    /// Constructs a new `BotCommand`.
    #[must_use]
    pub fn new(
        command: impl Into<Cow<'a, str>>,
        description: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
        }
    }
}

impl<'a> InteriorBorrow<'a> for BotCommand<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            command: self.command.borrow_inside(),
            description: self.description.borrow_inside(),
        }
    }
}
