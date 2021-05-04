use super::{InputFile, Thumb};
use crate::types::{
    file,
    parameters::{ParseMode, Text},
};
use serde::{ser::SerializeMap, Serializer};

/// Represents a document to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Document {
    pub(crate) media: InputFile,
    pub(crate) thumb: Option<Thumb>,
    pub(crate) caption: Option<String>,
    pub(crate) parse_mode: Option<ParseMode>,
    pub(crate) disable_content_type_detection: Option<bool>,
}

impl Document {
    const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            disable_content_type_detection: None,
        }
    }

    /// Constructs a `Document` from bytes.
    pub fn with_bytes(
        filename: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        Self::new(InputFile::File {
            filename: filename.into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `Document` from a file ID.
    ///
    /// # Panics
    ///
    /// Panics if the ID starts with `attach://`.
    pub fn with_id(id: file::Id) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot]: Document's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Document` from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the URL starts with `attach://`.
    pub fn with_url(url: impl Into<String>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Document's URL cannot start with `attach://`\n",
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

    /// Configures `disable_content_type_detection`.
    pub const fn should_disable_content_type_detection(
        mut self,
        should_disable: bool,
    ) -> Self {
        self.disable_content_type_detection = Some(should_disable);
        self
    }

    pub(crate) fn serialize_with_names<S>(
        &self,
        serializer: S,
        document_name: &str,
        thumb_name: &str,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("type", "document")?;
        map.serialize_entry("media", &self.media.with_name(document_name))?;

        if let Some(thumb) = &self.thumb {
            map.serialize_entry("thumb", &thumb.with_name(thumb_name))?;
        }
        if let Some(caption) = &self.caption {
            map.serialize_entry("caption", caption)?;
        }
        if let Some(parse_mode) = self.parse_mode {
            map.serialize_entry("parse_mode", &parse_mode)?;
        }
        if let Some(disable_ct_detection) = &self.disable_content_type_detection
        {
            map.serialize_entry(
                "disable_content_type_detection",
                disable_ct_detection,
            )?;
        }

        map.end()
    }
}

impl serde::Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialize_with_names(serializer, "document", "thumb")
    }
}
