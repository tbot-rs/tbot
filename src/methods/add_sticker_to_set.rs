use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        input_file::{InputFile, PngSticker},
        sticker::MaskPosition,
        user,
    },
};

/// Adds a new sticker to an existing sticker set.
///
/// Reflects the [`addStickerToSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#addstickertoset
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AddStickerToSet<'a, C> {
    client: &'a Client<C>,
    token: Token,
    user_id: user::Id,
    name: &'a str,
    png_sticker: PngSticker<'a>,
    emojis: &'a str,
    mask_position: Option<MaskPosition>,
}

impl<'a, C> AddStickerToSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
        name: &'a str,
        png_sticker: PngSticker<'a>,
        emojis: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            name,
            png_sticker,
            emojis,
            mask_position: None,
        }
    }

    /// Sets the mask's position. Reflects the `mask_position` parameter.
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
}

impl<C: Connector> AddStickerToSet<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(5)
            .string("user_id", &self.user_id)
            .str("name", self.name)
            .str("emojis", self.emojis)
            .maybe_json("mask_position", self.mask_position);

        match self.png_sticker.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("png_sticker", filename, bytes),
            InputFile::Id(sticker) | InputFile::Url(sticker) => {
                multipart = multipart.str("png_sticker", sticker);
            }
        }

        let (boundary, body) = multipart.finish();

        send_method::<bool, _>(
            self.client,
            &self.token,
            "addStickerToSet",
            Some(boundary),
            body,
        )
        .await?;

        Ok(())
    }
}
