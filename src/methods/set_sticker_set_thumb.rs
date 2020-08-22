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
    name: &'a str,
    thumb: Option<&'a StickerSetThumb<'a>>,
}

impl<'a> SetStickerSetThumb<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        user_id: user::Id,
        name: &'a str,
        thumb: Option<&'a StickerSetThumb<'a>>,
    ) -> Self {
        Self {
            bot,
            user_id,
            name,
            thumb,
        }
    }
}

impl SetStickerSetThumb<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(3)
            .string("user_id", &self.user_id)
            .str("name", self.name);

        if let Some(thumb) = self.thumb {
            match thumb.media {
                InputFile::File {
                    filename, bytes, ..
                } => multipart = multipart.file("thumb", filename, bytes),
                InputFile::Id(file::id::Ref(sticker))
                | InputFile::Url(sticker) => {
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
