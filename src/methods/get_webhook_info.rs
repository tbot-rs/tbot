use super::*;

/// Represents the [`getWebhookInfo`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getwebhookinfo
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetWebhookInfo {
    token: Token,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl GetWebhookInfo {
    /// Constructs a new `GetWebhookInfo`.
    pub const fn new(token: Token) -> Self {
        Self {
            token,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for GetWebhookInfo {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::WebhookInfo;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.token,
            "getWebhookInfo",
            None,
            Vec::new(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetWebhookInfo {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
