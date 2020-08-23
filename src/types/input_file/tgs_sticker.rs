use super::InputFile;
use crate::types::InteriorBorrow;
use std::borrow::Cow;

/// Represents a TGS sticker to be uploaded in a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct TgsSticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> TgsSticker<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `TgsSticker` from bytes.
    pub fn bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.tgs".into(),
            bytes: bytes.into(),
        })
    }
}

impl<'a> InteriorBorrow<'a> for TgsSticker<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            media: self.media.borrow_inside(),
        }
    }
}
