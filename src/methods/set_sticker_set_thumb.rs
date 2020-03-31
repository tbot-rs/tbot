use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    token,
    types::{
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
pub struct SetStickerSetThumb<'a, C> {
    client: &'a Client<C>,
    token: token::Ref<'a>,
    user_id: user::Id,
    name: &'a str,
    thumb: Option<&'a StickerSetThumb<'a>>,
}

impl<'a, C> SetStickerSetThumb<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        user_id: user::Id,
        name: &'a str,
        thumb: Option<&'a StickerSetThumb<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            name,
            thumb,
        }
    }
}

impl<C: Connector> SetStickerSetThumb<'_, C> {
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
                InputFile::Id(sticker) | InputFile::Url(sticker) => {
                    multipart = multipart.str("thumb", sticker);
                }
            }
        }

        let (boundary, body) = multipart.finish();

        send_method::<bool, _>(
            self.client,
            self.token,
            "setStickerSetThumb",
            Some(boundary),
            body,
        )
        .await?;

        Ok(())
    }
}
