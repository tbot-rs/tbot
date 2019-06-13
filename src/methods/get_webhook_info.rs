use super::*;
use crate::internal::Client;
use std::sync::Arc;

/// Represents the [`getWebhookInfo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getwebhookinfo
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetWebhookInfo<C> {
    client: Arc<Client<C>>,
    token: Token,
}

impl<C> GetWebhookInfo<C> {
    /// Constructs a new `GetWebhookInfo`.
    pub const fn new(client: Arc<Client<C>>, token: Token) -> Self {
        Self {
            client,
            token,
        }
    }
}

impl<C> IntoFuture for GetWebhookInfo<C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::WebhookInfo;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.client,
            &self.token,
            "getWebhookInfo",
            None,
            Vec::new(),
        ))
    }
}
