use super::InputFile;
use crate::types::parameters::{ParseMode, Text};
use serde::ser::SerializeMap;
use std::borrow::Cow;

/// Represents a photo to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Photo<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) caption: Option<Cow<'a, str>>,
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
    pub fn bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "photo.jpg".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `Photo` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: impl Into<Cow<'a, str>>) -> Self {
        let id = id.into();
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
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Photo's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text.into());
        self.parse_mode = caption.parse_mode;
        self
    }

    pub(crate) fn serialize<S>(
        &self,
        serializer: S,
        name: &str,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("type", "photo")?;
        map.serialize_entry("media", &self.media.with_name(name))?;

        if let Some(caption) = &self.caption {
            map.serialize_entry("caption", caption)?;
        }
        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }

        map.end()
    }
}

impl<'a> serde::Serialize for Photo<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.serialize(serializer, "photo")
    }
}
