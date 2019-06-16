use super::*;
use crate::types::parameters::ParseMode;
use serde::ser::SerializeMap;

/// Represents a photo to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Photo<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) caption: Option<&'a str>,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl<'a> Photo<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a `Photo` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "photo".into(),
            filename: "photo.jpg",
            bytes,
        })
    }

    /// Constructs a `Photo` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot]: Photo's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Photo` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Photo's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}

impl<'a> serde::Serialize for Photo<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "photo")?;
        map.serialize_entry("media", &self.media)?;

        if let Some(caption) = self.caption {
            map.serialize_entry("caption", caption)?;
        }
        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }

        map.end()
    }
}
