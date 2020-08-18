use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{user, File},
    Multipart,
};
use std::borrow::Cow;

/// Uploads a sticker file.
///
/// Reflects the [`uploadStickerFile`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#uploadstickerfile
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UploadStickerFile<'a> {
    bot: &'a InnerBot,
    user_id: user::Id,
    png_sticker: Cow<'a, [u8]>,
}

impl<'a> UploadStickerFile<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        user_id: user::Id,
        png_sticker: impl Into<Cow<'a, [u8]>>,
    ) -> Self {
        Self {
            bot,
            user_id,
            png_sticker: png_sticker.into(),
        }
    }
}

impl UploadStickerFile<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<File, errors::MethodCall> {
        let (boundary, body) = Multipart::new(2)
            .string("user_id", &self.user_id)
            .file("png_sticker", "sticker.png", &self.png_sticker)
            .finish();

        call_method(self.bot, "uploadStickerFile", Some(boundary), body).await
    }
}
