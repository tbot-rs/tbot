use super::*;

/// Represents the [`setStickerPositionInSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setstickerpositioninset
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetStickerPositionInSet<'a> {
    #[serde(skip)]
    token: &'a str,
    sticker: &'a str,
    position: u32,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
}

impl<'a> SetStickerPositionInSet<'a> {
    /// Constructs a new `SetStickerPositionInSet`.
    pub fn new(token: &'a str, sticker: &'a str, position: u32) -> Self {
        Self {
            token,
            sticker,
            position,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "setStickerPositionInSet",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ())
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SetStickerPositionInSet<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
