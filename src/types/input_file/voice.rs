use super::InputFile;
use crate::types::{
    file,
    parameters::{ParseMode, Text},
};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::borrow::Cow;

/// Represents a voice to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Voice<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) duration: Option<u32>,
    pub(crate) caption: Option<String>,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl<'a> Voice<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            duration: None,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a `Voice` from bytes.
    pub fn with_bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "voice.ogg".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `Voice` from a file ID.
    ///
    /// # Panics
    ///
    /// Panics if the ID starts with `attach://`.
    pub fn with_id(id: file::Id) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot]: Voice's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Voice` from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the URL starts with `attach://`.
    pub fn with_url(url: impl Into<Cow<'a, str>>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Voice's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `duration`.
    pub const fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: impl Into<Text>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }
}

impl<'a> Serialize for Voice<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("media", &self.media.with_name("voice"))?;

        if let Some(duration) = self.duration {
            map.serialize_entry("duration", &duration)?;
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
