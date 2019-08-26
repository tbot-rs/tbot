use super::*;
use crate::internal::{BoxFuture, Client};

/// Represents the [`getMe`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getme
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMe<'a, C> {
    client: &'a Client<C>,
    token: Token,
}

impl<'a, C> GetMe<'a, C> {
    pub(crate) const fn new(client: &'a Client<C>, token: Token) -> Self {
        Self { client, token }
    }
}

impl<C> IntoFuture for GetMe<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::User;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getMe",
            None,
            Vec::new(),
        ))
    }
}
