use super::call_method;
use crate::{bot::InnerBot, errors};

#[derive(Debug, Clone)]
#[must_use]
pub struct DeleteWebhook<'a> {
    bot: &'a InnerBot,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) const fn new(bot: &'a InnerBot) -> Self {
        Self { bot }
    }
}

impl DeleteWebhook<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(self.bot, "deleteWebhook", None, Vec::new())
            .await?;

        Ok(())
    }
}
