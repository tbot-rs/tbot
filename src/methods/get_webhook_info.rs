use super::*;

/// Representation of the [`getWebhookInfo`] method.
///
/// [`getWebhookInfo`]: https://core.telegram.org/bots/api#getwebhookinfo
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetWebhookInfo<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl<'a> GetWebhookInfo<'a> {
    /// Constructs a new `GetWebhookInfo`.
    pub fn new(token: &'a str) -> Self {
        Self {
            token,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::WebhookInfo, Error = DeliveryError> {
        send_method(
            self.token,
            "getWebhookInfo",
            None,
            Vec::new(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl<'a> ProxyMethod for GetWebhookInfo<'a> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
