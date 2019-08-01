use super::*;
use crate::types::{
    parameters::{ParseMode, Text},
    value::{self, Bytes, FileId, Ref},
};
use serde::ser::SerializeMap;

/// Represents a document to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Document<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<Ref<'a, Thumb<'a>>>,
    pub(crate) caption: Option<value::String<'a>>,
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
    pub fn bytes(
        filename: impl Into<value::String<'a>>,
        bytes: impl Into<Bytes<'a>>,
    ) -> Self {
        Self::new(InputFile::File {
            filename: filename.into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `Document` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: impl Into<FileId<'a>>) -> Self {
        let id = id.into();

        assert!(
            !id.as_ref().0.starts_with("attach://"),
            "\n[tbot]: Document's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Document` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: impl Into<value::String<'a>>) -> Self {
        let url = url.into();

        assert!(
            !url.as_str().starts_with("attach://"),
            "\n[tbot]: Document's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: impl Into<Ref<'a, Thumb<'a>>>) -> Self {
        self.thumb = Some(thumb.into());
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

impl serde::Serialize for Document<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "document")?;
        map.serialize_entry("media", &self.media.borrow_with_name("document"))?;

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }
        if let Some(caption) = &self.caption {
            map.serialize_entry("caption", &caption)?;
        }
        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }

        map.end()
    }
}
