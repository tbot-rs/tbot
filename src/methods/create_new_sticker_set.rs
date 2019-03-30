use super::*;
use types::input_file::{InputFile, PngSticker};
use types::MaskPosition;

/// Represents the [`createNewStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#createnewstickerset
#[must_use = "methods do nothing unless turned into a future"]
pub struct CreateNewStickerSet<'a> {
    token: &'a str,
    #[cfg(feature = "proxy")]
    proxy: Option<proxy::Proxy>,
    user_id: u64,
    name: &'a str,
    title: &'a str,
    png_sticker: &'a PngSticker<'a>,
    emojis: &'a str,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

impl<'a> CreateNewStickerSet<'a> {
    /// Constructs a new `CreateNewStickerSet`.
    pub fn new(
        token: &'a str,
        user_id: u64,
        name: &'a str,
        title: &'a str,
        png_sticker: &'a PngSticker<'a>,
        emojis: &'a str,
    ) -> Self {
        Self {
            token,
            user_id,
            name,
            title,
            png_sticker,
            emojis,
            contains_masks: None,
            mask_position: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `contains_mask`.
    pub fn contains_mask(mut self, contains_mask: bool) -> Self {
        self.contains_masks = Some(contains_mask);
        self
    }

    /// Configures `mask_position`.
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(self) -> impl Future<Item = (), Error = DeliveryError> {
        let user_id = self.user_id.to_string();
        let contains_mask = self.contains_masks.map(|x| x.to_string());
        let mask_position =
            self.mask_position.and_then(|x| serde_json::to_string(&x).ok());

        let mut multipart = Multipart::new(7)
            .str("user_id", &user_id)
            .str("name", &self.name)
            .str("title", &self.title)
            .str("emojis", &self.emojis)
            .maybe_string("contains_masks", &contains_mask)
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

        send_method::<bool>(
            self.token,
            "createNewStickerSet",
            Some(boundary),
            body,
            #[cfg(feature = "proxy")]
            self.proxy,
        )
        .map(|_| ())
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for CreateNewStickerSet<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
