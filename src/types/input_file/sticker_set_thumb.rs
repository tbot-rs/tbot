use super::InputFile;

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
    pub fn png_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "thumb.png",
            bytes,
        })
    }

    /// Constructs a `StickerSetThumb` from bytes of `.tgs` animation.
    pub fn tgs_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "thumb.tgs",
            bytes,
        })
    }

    /// Constructs a `StickerSetThumb` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot] StickerSetThumb's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `StickerSetThumb` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] StickerSetThumb's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
}
