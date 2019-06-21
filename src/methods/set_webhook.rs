use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::parameters::Updates,
};

/// This method isn't meant to be used by users directly.
#[derive(Debug, Clone)]
#[must_use]
pub(crate) struct SetWebhook<'a, C> {
    client: &'a Client<C>,
    token: Token,
    url: &'a str,
    certificate: Option<&'a str>,
    max_connections: Option<u8>,
    allowed_updates: Option<&'a [Updates]>,
}

impl<'a, C> SetWebhook<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        url: &'a str,
        certificate: Option<&'a str>,
        max_connections: Option<u8>,
        allowed_updates: Option<&'a [Updates]>,
    ) -> Self {
        Self {
            client,
            token,
            url,
            certificate,
            max_connections,
            allowed_updates,
        }
    }
}

impl<C> IntoFuture for SetWebhook<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let max_connections = self.max_connections.map(|x| x.to_string());
        let allowed_updates =
            self.allowed_updates.and_then(|x| serde_json::to_string(&x).ok());

        let (boundary, body) = Multipart::new(4)
            .str("url", self.url)
            .maybe_str("certificate", self.certificate)
            .maybe_string("max_connections", &max_connections)
            .maybe_string("allowed_updates", &allowed_updates)
            .finish();

        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "setWebhook",
                Some(boundary),
                body,
            )
            .map(|_| ()),
        )
    }
}
