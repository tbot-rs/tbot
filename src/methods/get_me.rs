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

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::User, Error = DeliveryError> {
        send_method(
            self.token,
            "getMe",
            None,
            Vec::new(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetMe<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
