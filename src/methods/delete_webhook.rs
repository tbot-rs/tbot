use super::*;

#[must_use]
pub struct DeleteWebhook<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl<'a> DeleteWebhook<'a> {
    #[cfg(not(feature = "proxy"))]
    pub const fn new(token: &'a str) -> Self {
        Self {
            token,
        }
    }

    #[cfg(feature = "proxy")]
    pub const fn new(token: &'a str, proxy: Option<proxy::Proxy>) -> Self {
        Self {
            token,
            proxy,
        }
    }
}

impl IntoFuture for DeleteWebhook<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                self.token,
                "deleteWebhook",
                None,
                Vec::new(),
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()),
        )
    }
}
