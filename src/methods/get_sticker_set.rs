use super::*;

/// Represents the [`getStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getstickerset
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetStickerSet<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    name: &'a str,
}

impl<'a> GetStickerSet<'a> {
    /// Constructs a new `GetStickerSet`.
    pub const fn new(token: &'a str, name: &'a str) -> Self {
        Self {
            token,
            name,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::StickerSet, Error = DeliveryError> {
        send_method(
            self.token,
            "getStickerSet",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for GetStickerSet<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
