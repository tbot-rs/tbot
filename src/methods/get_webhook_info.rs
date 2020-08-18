use super::call_method;
use crate::{bot::InnerBot, errors, types};

/// Gets information about the bot's webhook.
///
/// Reflects the [`getWebhookInfo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getwebhookinfo
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetWebhookInfo<'a> {
    bot: &'a InnerBot,
}

impl<'a> GetWebhookInfo<'a> {
    pub(crate) const fn new(bot: &'a InnerBot) -> Self {
        Self { bot }
    }
}

impl GetWebhookInfo<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<types::WebhookInfo, errors::MethodCall> {
        call_method(self.bot, "getWebhookInfo", None, Vec::new()).await
    }
}
