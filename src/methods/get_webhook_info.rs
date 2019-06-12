use super::*;

/// Represents the [`getWebhookInfo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getwebhookinfo
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetWebhookInfo<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl<'a> GetWebhookInfo<'a> {
    /// Constructs a new `GetWebhookInfo`.
    pub const fn new(token: &'a str) -> Self {
        Self {
            token,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for GetWebhookInfo<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::WebhookInfo;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.token,
            "getWebhookInfo",
            None,
            Vec::new(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetWebhookInfo<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
