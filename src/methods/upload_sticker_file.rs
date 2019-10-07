use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::user,
};

/// Uploads a sticker file.
///
/// Reflects the [`uploadStickerFile`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#uploadstickerfile
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UploadStickerFile<'a, C> {
    client: &'a Client<C>,
    token: Token,
    user_id: user::Id,
    png_sticker: &'a [u8],
}

impl<'a, C> UploadStickerFile<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
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

impl<C: Connector> IntoFuture for UploadStickerFile<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::File;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let (boundary, body) = Multipart::new(2)
            .string("user_id", &self.user_id)
            .file("png_sticker", "sticker.png", self.png_sticker)
            .finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "uploadStickerFile",
            Some(boundary),
            body,
        ))
    }
}
