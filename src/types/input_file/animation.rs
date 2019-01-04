use super::*;

/// Represents an animation to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Animation<'a> {
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
    pub(crate) duration: Option<u64>,
}

impl<'a> Animation<'a> {
    fn new(file: InputFile<'a>) -> Self {
        Self {
            file,
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
        }
    }

    /// Constructs an `Animation` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "animaion".into(),
            filename: "animation.mp4",
            bytes,
        })
    }

    /// Constructs an `Animation` from a file ID.
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: video note's ID cannot start with `attach://`",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs an `Animation` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: animation's URL cannot start with `attach://`",
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
    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);
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
}
