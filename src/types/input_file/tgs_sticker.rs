use super::InputFile;

/// Represents a TGS sticker to be uploaded in a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct TgsSticker {
    pub(crate) media: InputFile,
}

impl TgsSticker {
    const fn new(media: InputFile) -> Self {
        Self { media }
    }

    /// Constructs a `TgsSticker` from bytes.
    pub fn with_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.tgs".into(),
            bytes: bytes.into(),
        })
    }
}
