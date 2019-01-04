use super::*;

/// Represents a video to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Video<'a> {
    pub(crate) file: InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<InputFile<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<u64>,
}

impl<'a> Video<'a> {
    fn new(file: InputFile<'a>) -> Self {
        Self {
            file,
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
