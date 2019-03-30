use super::*;
use serde::ser::SerializeMap;

/// Represents a sticker to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Sticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> Sticker<'a> {
    fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
        }
    }

    /// Constructs a `Sticker` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "sticker".into(),
            filename: "sticker.webm",
            bytes,
        })
    }

    /// Constructs a `Sticker` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot] Sticker's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Sticker` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Sticker's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
}

impl<'a> serde::Serialize for Sticker<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "sticker")?;
        map.serialize_entry("media", &self.media)?;

        map.end()
    }
}
