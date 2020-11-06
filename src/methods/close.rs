use super::call_method;
use crate::{bot::InnerBot, errors};

/// Logs out from a self-hosted Bot API server.
///
/// Represents the [`Close`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#Close
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct Close<'a> {
    bot: &'a InnerBot,
}

impl<'a> Close<'a> {
    pub(crate) const fn new(bot: &'a InnerBot) -> Self {
        Self { bot }
    }
}

impl Close<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(self.bot, "close", None, Vec::new()).await?;
        Ok(())
    }
}
