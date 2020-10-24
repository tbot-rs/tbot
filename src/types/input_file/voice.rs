use super::{InputFile, WithName};
use crate::types::{
    file,
    parameters::{ParseMode, Text},
};
use serde::Serialize;

/// Represents a voice to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Voice<'a> {
    pub(crate) media: WithName<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
}

impl<'a> Voice<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self {
            media: media.with_name("voice"),
            duration: None,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a `Voice` from bytes.
    pub fn with_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "voice.ogg",
            bytes,
        })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_bytes`"
    )]
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::with_bytes(bytes)
    }

    /// Constructs a `Voice` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn with_id(id: file::id::Ref<'a>) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot]: Voice's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id.0))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `with_id` which takes a `file::id::Ref<'a>`"
    )]
    pub fn id(id: &'a str) -> Self {
        Self::with_id(file::id::Ref(id))
    }

    /// Constructs a `Voice` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn with_url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Voice's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_url`"
    )]
    pub fn url(url: &'a str) -> Self {
        Self::with_url(url)
    }

    /// Configures `duration`.
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text);
        self.parse_mode = caption.parse_mode;
        self
    }
}
