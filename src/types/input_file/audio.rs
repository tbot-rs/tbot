use super::*;
use crate::types::{
    parameters::{ParseMode, Text},
    value::{self, Bytes, FileId, Ref},
};
use serde::ser::SerializeMap;

/// Represents an audio to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Audio<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<Ref<'a, Thumb<'a>>>,
    pub(crate) caption: Option<value::String<'a>>,
    pub(crate) parse_mode: Option<ParseMode>,
    pub(crate) duration: Option<u32>,
    pub(crate) performer: Option<value::String<'a>>,
    pub(crate) title: Option<value::String<'a>>,
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
    pub fn bytes(bytes: impl Into<Bytes<'a>>) -> Self {
        Self::new(InputFile::File {
            filename: "audio.mp3".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs an `Audio` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: impl Into<FileId<'a>>) -> Self {
        let id = id.into();

        assert!(
            !id.as_ref().0.starts_with("attach://"),
            "\n[tbot] Audio's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs an `Audio` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: impl Into<value::String<'a>>) -> Self {
        let url = url.into();

        assert!(
            !url.as_str().starts_with("attach://"),
            "\n[tbot] Audio's URL cannot start with `attach://`\n",
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

    /// Configures `duration`.
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `performer`.
    pub fn performer(
        mut self,
        performer: impl Into<value::String<'a>>,
    ) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Configures `title`.
    pub fn title(mut self, title: impl Into<value::String<'a>>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl serde::Serialize for Audio<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "audio")?;
        map.serialize_entry("media", &self.media.borrow_with_name("audio"))?;

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }
        if let Some(caption) = &self.caption {
            map.serialize_entry("caption", &caption)?;
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
