use super::{InputFile, Thumb};
use crate::types::{
    file,
    parameters::{ParseMode, Text},
};
use serde::ser::SerializeMap;

/// Represents an animation to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Animation {
    pub(crate) media: InputFile,
    pub(crate) thumb: Option<Thumb>,
    pub(crate) caption: Option<String>,
    pub(crate) parse_mode: Option<ParseMode>,
    pub(crate) width: Option<u32>,
    pub(crate) height: Option<u32>,
    pub(crate) duration: Option<u32>,
}

impl Animation {
    const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
        }
    }

    /// Constructs an `Animation` from bytes.
    pub fn with_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(InputFile::File {
            filename: "animation.mp4".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs an `Animation` from a file ID.
    ///
    /// # Panics
    ///
    /// Panics if the ID starts with `attach://`.
    pub fn with_id(id: file::Id) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot] Animations's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs an `Animation` from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the URL starts with `attach://`.
    pub fn with_url(url: impl Into<String>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Animation's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    /// Configures `thumb`.
    #[allow(clippy::missing_const_for_fn)]
    pub fn thumb(mut self, thumb: Thumb) -> Self {
        self.thumb = Some(thumb);
        self
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: impl Into<Text>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }

    /// Configures `duration`.
    pub const fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `width`.
    pub const fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Configures `height`.
    pub const fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
}

impl serde::Serialize for Animation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "animation")?;
        map.serialize_entry("media", &self.media.with_name("animation"))?;

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb)?;
        }
        if let Some(caption) = &self.caption {
            map.serialize_entry("caption", caption)?;
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

        map.end()
    }
}
