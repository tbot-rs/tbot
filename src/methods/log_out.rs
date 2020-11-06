use super::call_method;
use crate::{bot::InnerBot, errors};

/// Logs out from the cloud Bot API server.
///
/// Represents the [`logOut`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#logout
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct LogOut<'a> {
    bot: &'a InnerBot,
}

impl<'a> LogOut<'a> {
    pub(crate) const fn new(bot: &'a InnerBot) -> Self {
        Self { bot }
    }
}

impl LogOut<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(self.bot, "logOut", None, Vec::new()).await?;
        Ok(())
    }
}
