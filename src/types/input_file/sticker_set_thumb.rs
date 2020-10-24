use super::InputFile;
use crate::types::file;

/// Represents a sticker set thumb to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct StickerSetThumb<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> StickerSetThumb<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `StickerSetThumb` from bytes of `.png` image.
    pub fn with_png_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "thumb.png",
            bytes,
        })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_png_bytes`"
    )]
    pub fn png_bytes(bytes: &'a [u8]) -> Self {
        Self::with_png_bytes(bytes)
    }

    /// Constructs a `StickerSetThumb` from bytes of `.tgs` animation.
    pub fn with_tgs_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "thumb.tgs",
            bytes,
        })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_tgs_bytes`"
    )]
    pub fn tgs_bytes(bytes: &'a [u8]) -> Self {
        Self::with_tgs_bytes(bytes)
    }

    /// Constructs a `StickerSetThumb` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn with_id(id: file::id::Ref<'a>) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot] StickerSetThumb's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id.0))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `with_id` which takes a `file::id::Ref<'a>`"
    )]
    pub fn id(id: &'a str) -> Self {
        Self::with_id(file::id::Ref(id))
    }

    /// Constructs a `StickerSetThumb` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn with_url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] StickerSetThumb's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_url`"
    )]
    pub fn url(url: &'a str) -> Self {
        Self::with_url(url)
    }
}
