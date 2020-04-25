use super::call_method;
use crate::{connectors::Client, errors, token, types};

/// Gets information about the bot's webhook.
///
/// Reflects the [`getWebhookInfo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getwebhookinfo
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetWebhookInfo<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
}

impl<'a> GetWebhookInfo<'a> {
    pub(crate) const fn new(client: &'a Client, token: token::Ref<'a>) -> Self {
        Self { client, token }
    }
}

impl GetWebhookInfo<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<types::WebhookInfo, errors::MethodCall> {
        call_method(self.client, self.token, "getWebhookInfo", None, Vec::new())
            .await
    }
}
