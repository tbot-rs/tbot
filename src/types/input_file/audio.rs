use super::*;
use serde::ser::SerializeMap;

/// Represents an audio to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Audio<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<InputFile<'a>>,
    pub(crate) caption: Option<&'a str>,
    pub(crate) parse_mode: Option<ParseMode>,
    pub(crate) duration: Option<u32>,
    pub(crate) performer: Option<&'a str>,
    pub(crate) title: Option<&'a str>,
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
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "audio".into(),
            filename: "audio.mp3",
            bytes,
        })
    }

    /// Constructs an `Audio` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot] Audio's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs an `Audio` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Audio's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: Thumb<'a>) -> Self {
        self.thumb = Some(thumb.0);
        self
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

    /// Configures `duration`.
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `performer`.
    pub fn performer(mut self, performer: &'a str) -> Self {
        self.performer = Some(performer);
        self
    }

    /// Configures `title`.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

impl<'a> serde::Serialize for Audio<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "audio")?;
        map.serialize_entry("media", &self.media)?;

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }
        if let Some(caption) = self.caption {
            map.serialize_entry("caption", caption)?;
        }
        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }
        if let Some(duration) = self.duration {
            map.serialize_entry("duration", &duration)?;
        }
        if let Some(performer) = self.performer {
            map.serialize_entry("performer", &performer)?;
        }
        if let Some(title) = self.title {
            map.serialize_entry("title", &title)?;
        }

        map.end()
    }
}
