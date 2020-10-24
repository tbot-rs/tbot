use super::InputFile;
use crate::types::file;

/// Represents a PNG sticker to be uploaded in a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct PngSticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> PngSticker<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `PngSticker` from bytes.
    pub fn with_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.png",
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

    /// Constructs a `PngSticker` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn with_id(id: file::id::Ref<'a>) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot] Sticker's ID cannot start with `attach://`\n",
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

    /// Constructs a `PngSticker` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn with_url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Sticker's URL cannot start with `attach://`\n",
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
