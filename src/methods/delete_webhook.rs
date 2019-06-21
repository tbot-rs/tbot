use super::*;
use crate::internal::{BoxFuture, Client};

#[derive(Debug, Clone)]
#[must_use]
pub struct DeleteWebhook<'a, C> {
    client: &'a Client<C>,
    token: Token,
}

impl<'a, C> DeleteWebhook<'a, C> {
    pub(crate) const fn new(client: &'a Client<C>, token: Token) -> Self {
        Self {
            client,
            token,
        }
    }
}

impl<C> IntoFuture for DeleteWebhook<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "deleteWebhook",
                None,
                Vec::new(),
            )
            .map(|_| ()),
        )
    }
}
