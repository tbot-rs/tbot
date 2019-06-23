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

/// Represents the [`addStickerToSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#addstickertoset
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct AddStickerToSet<'a, C> {
    client: &'a Client<C>,
    token: Token,
    user_id: user::Id,
    name: &'a str,
    png_sticker: &'a PngSticker<'a>,
    emojis: &'a str,
    mask_position: Option<MaskPosition>,
}

impl<'a, C> AddStickerToSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
        name: &'a str,
        png_sticker: &'a PngSticker<'a>,
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

    /// Configures `mask_position`.
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
}

impl<C> IntoFuture for AddStickerToSet<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(5)
            .string("user_id", &self.user_id)
            .str("name", self.name)
            .str("emojis", self.emojis)
            .maybe_json("mask_position", self.mask_position);

        match self.png_sticker.media {
            InputFile::File {
                filename,
                bytes,
                ..
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
                "addStickerToSet",
                Some(boundary),
                body,
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
