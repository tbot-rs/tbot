use super::*;

/// Represents the [`getMe`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getme
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMe<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl<'a> GetMe<'a> {
    /// Constructs a new `GetMe`.
    pub const fn new(token: &'a str) -> Self {
        Self {
            token,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for GetMe<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::User;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.token,
            "getMe",
            None,
            Vec::new(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetMe<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
