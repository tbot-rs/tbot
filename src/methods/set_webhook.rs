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
        let mut multipart = Multipart::new(4)
            .str("url", self.url)
            .maybe_string("max_connections", self.max_connections)
            .maybe_json("allowed_updates", self.allowed_updates);

        if let Some(certificate) = self.certificate {
            multipart = multipart.file(
                "certificate",
                "certificate.pem",
                certificate.as_bytes(),
            );
        }

        let (boundary, body) = multipart.finish();

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
