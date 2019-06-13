use super::*;
use crate::internal::Client;
use std::sync::Arc;

#[must_use]
pub struct DeleteWebhook<C> {
    client: Arc<Client<C>>,
    token: Token,
}

impl<C> DeleteWebhook<C> {
    pub(crate) const fn new(client: Arc<Client<C>>, token: Token) -> Self {
        Self {
            client,
            token,
        }
    }
}

impl<C> IntoFuture for DeleteWebhook<C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                &self.client,
                &self.token,
                "deleteWebhook",
                None,
                Vec::new(),
            )
            .map(|_| ()),
        )
    }
}
