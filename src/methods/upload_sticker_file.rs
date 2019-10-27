use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    token,
    types::{user, File},
    Multipart,
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
    token: token::Ref<'a>,
    user_id: user::Id,
    png_sticker: &'a [u8],
}

impl<'a, C> UploadStickerFile<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
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

impl<C: Connector> UploadStickerFile<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<File, errors::MethodCall> {
        let (boundary, body) = Multipart::new(2)
            .string("user_id", &self.user_id)
            .file("png_sticker", "sticker.png", self.png_sticker)
            .finish();

        send_method(
            self.client,
            self.token,
            "uploadStickerFile",
            Some(boundary),
            body,
        )
        .await
    }
}
