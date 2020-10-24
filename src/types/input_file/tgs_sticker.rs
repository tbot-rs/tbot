use super::InputFile;

/// Represents a TGS sticker to be uploaded in a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct TgsSticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> TgsSticker<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `TgsSticker` from bytes.
    pub fn with_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.tgs",
            bytes,
        })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_bytes`"
    )]
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::with_bytes(bytes)
    }
}
