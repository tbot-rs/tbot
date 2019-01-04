use super::*;

/// Represents an audio to be sent.
#[derive(Serialize)]
pub struct Audio<'a> {
    pub(crate) file: InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<InputFile<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) performer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<&'a str>,
}

impl<'a> Audio<'a> {
    fn new(file: InputFile<'a>) -> Self {
        Self {
            file,
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
            "tbot: audio's ID cannot start with `attach://`",
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
            "tbot: audio's URL cannot start with `attach://`",
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
