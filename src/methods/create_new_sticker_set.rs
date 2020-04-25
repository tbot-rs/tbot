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

/// Creates a new sticker set.
///
/// Reflects the [`createNewStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#createnewstickerset
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct CreateNewStickerSet<'a> {
    client: &'a Client,
    token: token::Ref<'a>,
    user_id: user::Id,
    name: &'a str,
    title: &'a str,
    sticker: StickerForStickerSet<'a>,
    emojis: &'a str,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

impl<'a> CreateNewStickerSet<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        user_id: user::Id,
        name: &'a str,
        title: &'a str,
        sticker: impl Into<StickerForStickerSet<'a>>,
        emojis: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            name,
            title,
            sticker: sticker.into(),
            emojis,
            contains_masks: None,
            mask_position: None,
        }
    }

    /// Configures if the sticker set is going to contain masks.
    /// Reflects the `contains_masks` parameter.
    pub fn contains_masks(mut self, contains_masks: bool) -> Self {
        self.contains_masks = Some(contains_masks);
        self
    }

    /// Configures the mask position of the first sticker.
    /// Reflects the `mask_position` parameter.
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
}

impl CreateNewStickerSet<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(7)
            .string("user_id", &self.user_id)
            .str("name", self.name)
            .str("title", self.title)
            .str("emojis", self.emojis)
            .maybe_string("contains_masks", self.contains_masks)
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
            "createNewStickerSet",
            Some(boundary),
            body,
        )
        .await?;

        Ok(())
    }
}
