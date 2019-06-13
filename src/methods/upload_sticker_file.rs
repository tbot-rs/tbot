use super::*;
use std::sync::Arc;

/// Represents the [`uploadStickerFile`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#uploadstickerfile
#[must_use = "methods do nothing unless turned into a future"]
pub struct UploadStickerFile<'a, C> {
    client: Arc<hyper::Client<C, hyper::Body>>,
    token: Token,
    user_id: i64,
    png_sticker: &'a [u8],
}

impl<'a, C> UploadStickerFile<'a, C> {
    /// Constructs a new `UploadStickerFile`.
    pub const fn new(
        client: Arc<hyper::Client<C, hyper::Body>>,
        token: Token,
        user_id: i64,
        png_sticker: &'a [u8],
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            png_sticker,
        }
    }
}

impl<C> IntoFuture for UploadStickerFile<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::File;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let (boundary, body) = Multipart::new(2)
            .str("user_id", &self.user_id.to_string())
            .file("png_sticker", "sticker.png", self.png_sticker)
            .finish();

        Box::new(send_method(
            &self.client,
            &self.token,
            "uploadStickerFile",
            Some(boundary),
            body,
        ))
    }
}
