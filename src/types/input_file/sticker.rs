use super::InputFile;
use crate::types::{file, InteriorBorrow};
use serde::ser::SerializeMap;
use std::borrow::Cow;

/// Represents a sticker to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Sticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> Sticker<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `Sticker` from bytes.
    pub fn bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
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
    pub fn id(id: file::Id<'a>) -> Self {
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
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Sticker's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
}

impl<'a> InteriorBorrow<'a> for Sticker<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            media: self.media.borrow_inside(),
        }
    }
}

impl<'a> serde::Serialize for Sticker<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "sticker")?;
        map.serialize_entry("media", &self.media.with_name("sticker"))?;

        map.end()
    }
}
