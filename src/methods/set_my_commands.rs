use super::call_method;
use crate::{bot::InnerBot, errors, types::BotCommand};
use serde::Serialize;

/// Sets the list of the bot's commands.
///
/// Represents the [`setMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setmycommands
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetMyCommands<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    commands: Vec<BotCommand>,
}

impl<'a> SetMyCommands<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        commands: impl Into<Vec<BotCommand>>,
    ) -> Self {
        Self {
            bot,
            commands: commands.into(),
        }
    }
}

impl SetMyCommands<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setMyCommands",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
