use super::call_method;
use crate::{bot::InnerBot, errors, types};

/// Gets information about the bot.
///
/// Represents the [`getMe`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getme
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMe<'a> {
    bot: &'a InnerBot,
}

impl<'a> GetMe<'a> {
    pub(crate) const fn new(bot: &'a InnerBot) -> Self {
        Self { bot }
    }
}

impl GetMe<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<types::user::Me, errors::MethodCall> {
        call_method(self.bot, "getMe", None, Vec::new()).await
    }
}
