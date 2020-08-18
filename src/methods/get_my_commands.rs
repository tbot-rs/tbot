use super::call_method;
use crate::{bot::InnerBot, errors, types::BotCommand};

/// Gets the list of the bot's commands.
///
/// Represents the [`getMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getmycommands
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMyCommands<'a> {
    bot: &'a InnerBot,
}

impl<'a> GetMyCommands<'a> {
    pub(crate) const fn new(bot: &'a InnerBot) -> Self {
        Self { bot }
    }
}

impl GetMyCommands<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<BotCommand>, errors::MethodCall> {
        call_method(self.bot, "getMyCommands", None, Vec::new()).await
    }
}
