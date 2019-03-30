use super::*;

/// Represents the [`uploadStickerFile`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#uploadstickerfile
#[must_use = "methods do nothing unless turned into a future"]
pub struct UploadStickerFile<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    user_id: u64,
    png_sticker: &'a [u8],
}

impl<'a> UploadStickerFile<'a> {
    /// Constructs a new `UploadStickerFile`.
    pub fn new(token: &'a str, user_id: u64, png_sticker: &'a [u8]) -> Self {
        Self {
            token,
            user_id,
            png_sticker,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::File, Error = DeliveryError> {
        let (boundary, body) = Multipart::new(2)
            .str("user_id", &self.user_id.to_string())
            .file("png_sticker", "sticker.png", self.png_sticker)
            .finish();

        send_method(
            self.token,
            "uploadStickerFile",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for UploadStickerFile<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
