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

    #[must_use]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "deleteWebhook",
            None,
            Vec::new(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ())
    }
}
