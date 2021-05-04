use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        file,
        input_file::{InputFile, StickerSetThumb},
        user,
    },
    Multipart,
};

/// Sets the thumb of a sticker set.
///
/// Reflects the [`setStickerSetThumb`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setstickersetthumb
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetStickerSetThumb<'a> {
    bot: &'a InnerBot,
    user_id: user::Id,
    name: String,
    thumb: Option<StickerSetThumb>,
}

impl<'a> SetStickerSetThumb<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        user_id: user::Id,
        name: impl Into<String>,
        thumb: Option<StickerSetThumb>,
    ) -> Self {
        Self {
            bot,
            user_id,
            name: name.into(),
            thumb,
        }
    }
}

impl SetStickerSetThumb<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(3)
            .string("user_id", &self.user_id)
            .str("name", &self.name);

        if let Some(thumb) = &self.thumb {
            match &thumb.media {
                InputFile::File {
                    filename, bytes, ..
                } => multipart = multipart.file("thumb", filename, bytes),
                InputFile::Id(file::Id(sticker)) | InputFile::Url(sticker) => {
                    multipart = multipart.str("thumb", sticker);
                }
            }
        }

        let (boundary, body) = multipart.finish();

        call_method::<bool>(
            self.bot,
            "setStickerSetThumb",
            Some(boundary),
            body,
        )
        .await?;

        Ok(())
    }
}
