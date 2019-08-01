use super::{InputFile, Thumb, WithName};
use crate::types::value::{self, Bytes, FileId, Ref};
use serde::Serialize;

/// Represents a video note to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct VideoNote<'a> {
    pub(crate) media: WithName<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<Ref<'a, Thumb<'a>>>,
}

impl<'a> VideoNote<'a> {
    fn new(media: InputFile<'a>) -> Self {
        Self {
            media: media.own_with_name("video_note"),
            duration: None,
            length: None,
            thumb: None,
        }
    }

    /// Constructs an `VideoNote` from bytes.
    pub fn bytes(bytes: impl Into<Bytes<'a>>) -> Self {
        Self::new(InputFile::File {
            filename: "video_note.mp4".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `VideoNote` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: impl Into<FileId<'a>>) -> Self {
        let id = id.into();

        assert!(
            !id.as_ref().0.starts_with("attach://"),
            "\n[tbot]: Video note's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `VideoNote` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: impl Into<value::String<'a>>) -> Self {
        let url = url.into();
        assert!(
            !url.as_str().starts_with("attach://"),
            "\n[tbot]: Video note's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `duration`.
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `length`.
    pub fn length(mut self, length: u32) -> Self {
        self.length = Some(length);
        self
    }

    /// Configures `thumb`.
    pub fn thumb(mut self, thumb: impl Into<Ref<'a, Thumb<'a>>>) -> Self {
        self.thumb = Some(thumb.into());
        self
    }
}
