use super::{InputFile, Thumb};
use crate::types::{
    file,
    parameters::{ParseMode, Text},
};
use serde::ser::SerializeMap;
use std::borrow::Cow;

/// Represents an audio to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Audio<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<Thumb<'a>>,
    pub(crate) caption: Option<Cow<'a, str>>,
    pub(crate) parse_mode: Option<ParseMode>,
    pub(crate) duration: Option<u32>,
    pub(crate) performer: Option<Cow<'a, str>>,
    pub(crate) title: Option<Cow<'a, str>>,
}

impl<'a> Audio<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
        }
    }

    /// Constructs an `Audio` from bytes.
    pub fn bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "audio.mp3".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs an `Audio` from a file ID.
    ///
    /// # Panics
    ///
    /// Panics if the ID starts with `attach://`.
    pub fn id(id: file::Id<'a>) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot] Audio's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs an `Audio` from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the URL starts with `attach://`.
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Audio's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `thumb`.
    #[allow(clippy::missing_const_for_fn)]
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

    /// Configures `duration`.
    pub const fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `performer`.
    pub fn performer(mut self, performer: impl Into<Cow<'a, str>>) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Configures `title`.
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl<'a> serde::Serialize for Audio<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "audio")?;
        map.serialize_entry("media", &self.media.with_name("audio"))?;

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }
        if let Some(caption) = &self.caption {
            map.serialize_entry("caption", caption)?;
        }
        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }
        if let Some(duration) = self.duration {
            map.serialize_entry("duration", &duration)?;
        }
        if let Some(performer) = &self.performer {
            map.serialize_entry("performer", &performer)?;
        }
        if let Some(title) = &self.title {
            map.serialize_entry("title", &title)?;
        }

        map.end()
    }
}
