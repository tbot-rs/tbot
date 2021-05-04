use super::{InputFile, Thumb};
use crate::types::file;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

/// Represents a video note to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct VideoNote {
    pub(crate) media: InputFile,
    pub(crate) duration: Option<u32>,
    pub(crate) length: Option<u32>,
    pub(crate) thumb: Option<Thumb>,
}

impl VideoNote {
    const fn new(media: InputFile) -> Self {
        Self {
            media,
            duration: None,
            length: None,
            thumb: None,
        }
    }

    /// Constructs an `VideoNote` from bytes.
    pub fn with_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(InputFile::File {
            filename: "video_note.mp4".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `VideoNote` from a file ID.
    ///
    /// # Panics
    ///
    /// Panics if the ID starts with `attach://`.
    pub fn with_id(id: file::Id) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot]: Video note's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `VideoNote` from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the URL starts with `attach://`.
    pub fn with_url(url: impl Into<String>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Video note's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `duration`.
    pub const fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `length`.
    pub const fn length(mut self, length: u32) -> Self {
        self.length = Some(length);
        self
    }

    /// Configures `thumb`.
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumb(mut self, thumb: Thumb) -> Self {
        self.thumb = Some(thumb);
        self
    }
}

impl Serialize for VideoNote {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("media", &self.media.with_name("video_note"))?;

        if let Some(duration) = self.duration {
            map.serialize_entry("duration", &duration)?;
        }

        if let Some(length) = self.length {
            map.serialize_entry("length", &length)?;
        }

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }

        map.end()
    }
}
