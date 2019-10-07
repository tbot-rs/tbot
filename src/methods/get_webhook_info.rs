use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    prelude::IntoFuture,
    types, Token,
};

/// Gets information about the bot's webhook.
///
/// Reflects the [`getWebhookInfo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getwebhookinfo
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetWebhookInfo<'a, C> {
    client: &'a Client<C>,
    token: Token,
}

impl<'a, C> GetWebhookInfo<'a, C> {
    pub(crate) const fn new(client: &'a Client<C>, token: Token) -> Self {
        Self { client, token }
    }
}

impl<C: Connector> IntoFuture for GetWebhookInfo<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::WebhookInfo;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getWebhookInfo",
            None,
            Vec::new(),
        ))
    }
}
