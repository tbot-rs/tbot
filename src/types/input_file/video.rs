use super::*;
use serde::ser::SerializeMap;

/// Represents a video to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Video<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<InputFile<'a>>,
    pub(crate) caption: Option<&'a str>,
    pub(crate) parse_mode: Option<ParseMode>,
    pub(crate) width: Option<u64>,
    pub(crate) height: Option<u64>,
    pub(crate) supports_streaming: Option<bool>,
    pub(crate) duration: Option<u64>,
}

impl<'a> Video<'a> {
    fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            supports_streaming: None,
            duration: None,
        }
    }

    /// Constructs a `Video` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "video".into(),
            filename: "video.mp4",
            bytes,
        })
    }

    /// Constructs a `Video` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: video's ID cannot start with `attach://`",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Video` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: video's URL cannot start with `attach://`",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: types::Thumb<'a>) -> Self {
        self.thumb = Some(thumb.0);
        self
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, mode: types::ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Configures `width`.
    pub fn width(mut self, width: u64) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures `height`.
    pub fn height(mut self, height: u64) -> Self {
        self.height = Some(height);
        self
    }

    /// Configures `duration`.
    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `supports_streaming`.
    pub fn supports_streaming(mut self, is_streamed: bool) -> Self {
        self.supports_streaming = Some(is_streamed);
        self
    }
}

impl<'a> serde::Serialize for Video<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "video")?;
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
        if let Some(width) = self.width {
            map.serialize_entry("width", &width)?;
        }
        if let Some(height) = self.height {
            map.serialize_entry("height", &height)?;
        }
        if let Some(has_support) = self.supports_streaming {
            map.serialize_entry("supports_streaming", &has_support)?;
        }

        map.end()
    }
}