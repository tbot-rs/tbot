use super::*;

/// Representation of the [`getMe`] method.
///
/// [`getMe`]: https://core.telegram.org/bots/api#getme
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMe<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
}

impl<'a> GetMe<'a> {
    /// Constructs a new `GetMe`.
    pub fn new<'b: 'a>(token: &'b str) -> Self {
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
impl<'a> ProxyMethod for GetMe<'a> {
    /// Configures `proxy`.
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
