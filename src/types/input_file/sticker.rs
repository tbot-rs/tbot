use super::InputFile;
use crate::types::file;
use serde::ser::SerializeMap;

/// Represents a sticker to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Sticker {
    pub(crate) media: InputFile,
}

impl Sticker {
    const fn new(media: InputFile) -> Self {
        Self { media }
    }

    /// Constructs a `Sticker` from bytes.
    pub fn with_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.webm".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `Sticker` from a file ID.
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

    /// Constructs a `Sticker` from an URL.
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
impl serde::Serialize for Sticker {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "sticker")?;
        map.serialize_entry("media", &self.media.with_name("sticker"))?;

        map.end()
    }
}
