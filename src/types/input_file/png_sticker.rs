use super::InputFile;
use crate::types::file;

/// Represents a PNG sticker to be uploaded in a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct PngSticker {
    pub(crate) media: InputFile,
}

impl PngSticker {
    const fn new(media: InputFile) -> Self {
        Self { media }
    }

    /// Constructs a `PngSticker` from bytes.
    pub fn with_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.png".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `PngSticker` from a file ID.
    ///
    /// # Panics
    ///
    /// Panics if the ID starts with `attach://`.
    pub fn with_id(id: file::Id) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot] Sticker's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `PngSticker` from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the URL starts with `attach://`.
    pub fn with_url(url: impl Into<String>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Sticker's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
}
