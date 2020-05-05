use super::InputFile;
use std::borrow::Cow;

/// Represents a PNG sticker to be uploaded in a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct PngSticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> PngSticker<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `PngSticker` from bytes.
    pub fn bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.png".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `PngSticker` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: impl Into<Cow<'a, str>>) -> Self {
        let id = id.into();
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot] Sticker's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `PngSticker` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Sticker's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
}
