use super::InputFile;
use crate::types::{file, InteriorBorrow};
use std::borrow::Cow;

/// Represents a sticker set thumb to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct StickerSetThumb<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> StickerSetThumb<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `StickerSetThumb` from bytes of `.png` image.
    pub fn png_bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "thumb.png".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `StickerSetThumb` from bytes of `.tgs` animation.
    pub fn tgs_bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "thumb.tgs".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `StickerSetThumb` from a file ID.
    ///
    /// # Panics
    ///
    /// Panics if the ID starts with `attach://`.
    pub fn id(id: file::Id<'a>) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot] StickerSetThumb's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `StickerSetThumb` from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the URL starts with `attach://`.
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] StickerSetThumb's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
}

impl<'a> InteriorBorrow<'a> for StickerSetThumb<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            media: self.media.borrow_inside(),
        }
    }
}
