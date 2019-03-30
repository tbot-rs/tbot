use super::*;

/// Represents a video note to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct VideoNote<'a> {
    pub(crate) media: InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<InputFile<'a>>,
}

impl<'a> VideoNote<'a> {
    fn new(media: InputFile<'a>) -> Self {
        Self {
            media,
            duration: None,
            length: None,
            thumb: None,
        }
    }

    /// Constructs an `VideoNote` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "video_note".into(),
            filename: "video_note.mp4",
            bytes,
        })
    }

    /// Constructs a `VideoNote` from a file ID.
    ///
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

    /// Constructs a `VideoNote` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: video note's URL cannot start with `attach://`",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `duration`.
    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `length`.
    pub fn length(mut self, length: u64) -> Self {
        self.length = Some(length);
        self
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: super::Thumb<'a>) -> Self {
        self.thumb = Some(thumb.0);
        self
    }
}
