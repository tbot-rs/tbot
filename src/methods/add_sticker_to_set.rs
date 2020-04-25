use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        input_file::{InputFile, StickerForStickerSet},
        sticker::MaskPosition,
        user,
    },
    Multipart,
};

/// Adds a new sticker to an existing sticker set.
///
/// Reflects the [`addStickerToSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#addstickertoset
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AddStickerToSet<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    user_id: user::Id,
    name: &'a str,
    sticker: StickerForStickerSet<'a>,
    emojis: &'a str,
    mask_position: Option<MaskPosition>,
}

impl<'a> AddStickerToSet<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        user_id: user::Id,
        name: &'a str,
        sticker: impl Into<StickerForStickerSet<'a>>,
        emojis: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            name,
            sticker: sticker.into(),
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

impl AddStickerToSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(5)
            .string("user_id", &self.user_id)
            .str("name", self.name)
            .str("emojis", self.emojis)
            .maybe_json("mask_position", self.mask_position);

        let (field, media) = match self.sticker {
            StickerForStickerSet::Png(sticker) => {
                ("png_sticker", sticker.media)
            }
            StickerForStickerSet::Tgs(sticker) => {
                ("tgs_sticker", sticker.media)
            }
        };

        match media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file(field, filename, bytes),
            InputFile::Id(sticker) | InputFile::Url(sticker) => {
                multipart = multipart.str(field, sticker);
            }
        }

        let (boundary, body) = multipart.finish();

        call_method::<bool>(
            self.client,
            self.token,
            "addStickerToSet",
            Some(boundary),
            body,
        )
        .await?;

        Ok(())
    }
}
