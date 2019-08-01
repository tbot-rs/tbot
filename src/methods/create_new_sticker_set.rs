use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::{InputFile, PngSticker},
        sticker::MaskPosition,
        user,
        value::{self, Ref},
    },
};

/// Represents the [`createNewStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#createnewstickerset
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct CreateNewStickerSet<'a, C> {
    client: &'a Client<C>,
    token: Token,
    user_id: user::Id,
    name: value::String<'a>,
    title: value::String<'a>,
    png_sticker: Ref<'a, PngSticker<'a>>,
    emojis: value::String<'a>,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

impl<'a, C> CreateNewStickerSet<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
        name: impl Into<value::String<'a>>,
        title: impl Into<value::String<'a>>,
        png_sticker: impl Into<Ref<'a, PngSticker<'a>>>,
        emojis: impl Into<value::String<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            name: name.into(),
            title: title.into(),
            png_sticker: png_sticker.into(),
            emojis: emojis.into(),
            contains_masks: None,
            mask_position: None,
        }
    }

    /// Configures `contains_masks`.
    pub fn contains_masks(mut self, contains_masks: bool) -> Self {
        self.contains_masks = Some(contains_masks);
        self
    }

    /// Configures `mask_position`.
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
            .str("user_id", self.user_id.to_string())
            .str("name", self.name)
            .str("title", self.title)
            .str("emojis", self.emojis)
            .maybe_from("contains_masks", self.contains_masks)
            .maybe_json("mask_position", self.mask_position);

        match &self.png_sticker.as_ref().media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("png_sticker", filename, bytes),
            InputFile::Id(id) => {
                multipart = multipart.str("png_sticker", id.as_ref().0);
            }
            InputFile::Url(url) => {
                multipart = multipart.str("png_sticker", url);
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
