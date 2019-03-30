use super::*;

/// Represents the [`deleteStickerFromSet`][docs] method
///
/// [docs]: https://core.telegram.org/bots/api#deletestickerfromset
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct DeleteStickerFromSet<'a> {
    #[serde(skip)]
    token: &'a str,
    sticker: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
}

impl<'a> DeleteStickerFromSet<'a> {
    /// Constructs a new `DeleteStickerFromSet`.
    pub fn new(token: &'a str, sticker: &'a str) -> Self {
        Self {
            token,
            sticker,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "deleteStickerFromSet",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ())
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for DeleteStickerFromSet<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
