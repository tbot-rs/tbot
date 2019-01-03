use super::*;
use serde::ser::SerializeMap;

/// Represents an [`InputMediaVideo`].
///
/// [`InputMediaVideo`]: https://core.telegram.org/bots/api#inputmediaphoto
pub struct InputMediaVideo<'a> {
    pub(crate) media: InputFile<'a>,
    pub(crate) thumb: Option<types::InputFile<'a>>,
    caption: Option<&'a str>,
    parse_mode: Option<ParseMode>,
    width: Option<u64>,
    height: Option<u64>,
    duration: Option<u64>,
    supports_streaming: Option<bool>,
}

impl<'a> InputMediaVideo<'a> {
    pub(crate) fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }

    /// Constructs a new `InputMediaVideo` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "video".into(),
            filename: "video.mp4",
            bytes,
        })
    }

    /// Constructs a new `InputMediaVideo` from a file ID.
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

    /// Constructs a new `InputMediaVideo` from a URL.
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
    pub fn supports_streaming(mut self, has_support: bool) -> Self {
        self.supports_streaming = Some(has_support);
        self
    }
}

impl<'a> serde::Serialize for InputMediaVideo<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "video")?;
        map.serialize_entry("media", &self.media)?;

        if let Some(caption) = self.caption {
            map.serialize_entry("caption", caption)?;
        }

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }

        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }

        if let Some(width) = self.width {
            map.serialize_entry("width", &width)?;
        }

        if let Some(height) = self.height {
            map.serialize_entry("height", &height)?;
        }

        if let Some(duration) = self.duration {
            map.serialize_entry("duration", &duration)?;
        }

        if let Some(supports_streaming) = self.supports_streaming {
            map.serialize_entry("supports_streaming", &supports_streaming)?;
        }

        map.end()
    }
}
