use super::*;

/// This method isn't meant to be used by users directly.
#[must_use]
pub(crate) struct SetWebhook<'a> {
    token: &'a str,
    url: &'a str,
    certificate: Option<&'a str>,
    max_connections: Option<u8>,
    allowed_updates: Option<&'a [types::Updates]>,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl<'a> SetWebhook<'a> {
    #[cfg(feature = "proxy")]
    pub const fn new(
        token: &'a str,
        url: &'a str,
        certificate: Option<&'a str>,
        max_connections: Option<u8>,
        allowed_updates: Option<&'a [types::Updates]>,
        proxy: Option<proxy::Proxy>,
    ) -> Self {
        Self {
            token,
            url,
            certificate,
            max_connections,
            allowed_updates,
            proxy,
        }
    }

    #[cfg(not(feature = "proxy"))]
    pub const fn new(
        token: &'a str,
        url: &'a str,
        certificate: Option<&'a str>,
        max_connections: Option<u8>,
        allowed_updates: Option<&'a [types::Updates]>,
    ) -> Self {
        Self {
            token,
            url,
            certificate,
            max_connections,
            allowed_updates,
        }
    }
}

impl IntoFuture for SetWebhook<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

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
            send_method::<bool>(
                self.token,
                "setWebhook",
                Some(boundary),
                body,
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()),
        )
    }
}
