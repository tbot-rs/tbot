use super::{InputFile, Thumb};
use crate::types::parameters::{ParseMode, Text};
use serde::ser::SerializeMap;

/// Represents a video to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct Video<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<Thumb<'a>>,
    pub(crate) caption: Option<&'a str>,
    pub(crate) parse_mode: Option<ParseMode>,
    pub(crate) width: Option<u32>,
    pub(crate) height: Option<u32>,
    pub(crate) supports_streaming: Option<bool>,
    pub(crate) duration: Option<u32>,
}

impl<'a> Video<'a> {
    const fn new(media: InputFile<'a>) -> Self {
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
            "\n[tbot]: Video's ID cannot start with `attach://`\n",
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
            "\n[tbot]: Video's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: super::Thumb<'a>) -> Self {
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

    /// Configures `width`.
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures `height`.
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Configures `duration`.
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `supports_streaming`.
    pub fn supports_streaming(mut self, is_streamed: bool) -> Self {
        self.supports_streaming = Some(is_streamed);
        self
    }

    pub(crate) fn serialize<S>(
        &self,
        serialize: S,
        video_name: &str,
        thumb_name: &str,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serialize.serialize_map(None)?;

        map.serialize_entry("type", "video")?;
        map.serialize_entry("media", &self.media.with_name(video_name))?;

        if let Some(thumb) = self.thumb {
            map.serialize_entry("thumb", &thumb.with_name(thumb_name))?;
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

impl<'a> serde::Serialize for Video<'a> {
    fn serialize<S>(&self, serialize: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.serialize(serialize, "video", "thumb")
    }
}
