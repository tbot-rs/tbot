use super::InputFile;
use crate::types::parameters::ParseMode;
use serde::Serialize;

/// Represents a voice to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Voice<'a> {
    pub(crate) media: InputFile<'a>,
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
            media,
            duration: None,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a `Voice` from bytes.
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "voice".into(),
            filename: "voice.ogg",
            bytes,
        })
    }

    /// Constructs a `Voice` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot]: Voice's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Voice` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Voice's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
    /// Configures `duration`.
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Configures `parse_mode`.
    pub fn parse_mode(mut self, mode: ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }
}
