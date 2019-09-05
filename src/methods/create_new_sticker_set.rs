use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{InputFile, PngSticker},
        sticker::MaskPosition,
        user,
    },
};

/// Creates a new sticker set.
///
/// Reflects the [`createNewStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#createnewstickerset
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct CreateNewStickerSet<'a, C> {
    client: &'a Client<C>,
    token: Token,
    user_id: user::Id,
    name: &'a str,
    title: &'a str,
    png_sticker: PngSticker<'a>,
    emojis: &'a str,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

impl<'a, C> CreateNewStickerSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
        name: &'a str,
        title: &'a str,
        png_sticker: PngSticker<'a>,
        emojis: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            name,
            title,
            png_sticker,
            emojis,
            contains_masks: None,
            mask_position: None,
        }
    }

    /// Configures if the sticker pack is going to contain masks.
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

impl<C> IntoFuture for CreateNewStickerSet<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(7)
            .string("user_id", &self.user_id)
            .str("name", self.name)
            .str("title", self.title)
            .str("emojis", self.emojis)
            .maybe_string("contains_masks", self.contains_masks)
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

        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "createNewStickerSet",
                Some(boundary),
                body,
            )
            .map(|_| ()),
        )
    }
}
