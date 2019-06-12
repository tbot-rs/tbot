use super::*;

#[must_use]
pub struct DeleteWebhook {
    token: Token,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl DeleteWebhook {
    #[cfg(not(feature = "proxy"))]
    pub const fn new(token: Token) -> Self {
        Self {
            token,
        }
    }

    #[cfg(feature = "proxy")]
    pub const fn new(token: Token, proxy: Option<proxy::Proxy>) -> Self {
        Self {
            token,
            proxy,
        }
    }
}

impl IntoFuture for DeleteWebhook {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                &self.token,
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
