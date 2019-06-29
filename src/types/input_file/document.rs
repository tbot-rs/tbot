use super::*;
use crate::types::parameters::{ParseMode, Text};
use serde::ser::SerializeMap;

/// Represents a document to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Document<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<Thumb<'a>>,
    pub(crate) caption: Option<&'a str>,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl<'a> Document<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a `Document` from bytes.
    pub fn bytes(filename: &'a str, bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename,
            bytes,
        })
    }

    /// Constructs a `Document` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot]: Document's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Document` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Document's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb);
        self
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }
}

impl<'a> serde::Serialize for Document<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "document")?;
        map.serialize_entry("media", &self.media.with_name("document"))?;

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }
        if let Some(caption) = self.caption {
            map.serialize_entry("caption", caption)?;
        }
        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }

        map.end()
    }
}
