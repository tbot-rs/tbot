use super::*;
use crate::internal::Client;
use types::input_file::{InputFile, PngSticker};
use types::MaskPosition;

/// Represents the [`addStickerToSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#addstickertoset
#[must_use = "methods do nothing unless turned into a future"]
pub struct AddStickerToSet<'a, C> {
    client: &'a Client<C>,
    token: Token,
    user_id: i64,
    name: &'a str,
    png_sticker: &'a PngSticker<'a>,
    emojis: &'a str,
    mask_position: Option<MaskPosition>,
}

impl<'a, C> AddStickerToSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: i64,
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
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let user_id = self.user_id.to_string();
        let mask_position =
            self.mask_position.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(5)
            .str("user_id", &user_id)
            .str("name", self.name)
            .str("emojis", self.emojis)
            .maybe_string("mask_position", &mask_position);

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
                &self.client,
                &self.token,
                "addStickerToSet",
                Some(boundary),
                body,
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}
