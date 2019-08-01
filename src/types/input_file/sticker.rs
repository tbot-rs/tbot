use super::*;
use crate::types::value::{self, Bytes, FileId};
use serde::ser::SerializeMap;

/// Represents a sticker to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Sticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> Sticker<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
        }
    }

    /// Constructs a `Sticker` from bytes.
    pub fn bytes(bytes: impl Into<Bytes<'a>>) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.webm".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `Sticker` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: impl Into<FileId<'a>>) -> Self {
        let id = id.into();

        assert!(
            !id.as_ref().0.starts_with("attach://"),
            "\n[tbot] Sticker's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Sticker` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: impl Into<value::String<'a>>) -> Self {
        let url = url.into();

        assert!(
            !url.as_str().starts_with("attach://"),
            "\n[tbot] Sticker's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
}

impl serde::Serialize for Sticker<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "sticker")?;
        map.serialize_entry("media", &self.media.borrow_with_name("sticker"))?;

        map.end()
    }
}
