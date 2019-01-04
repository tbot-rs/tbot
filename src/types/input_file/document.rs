use super::*;

/// Represents a document to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Document<'a> {
    pub(crate) file: InputFile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<InputFile<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
}

impl<'a> Document<'a> {
    fn new(file: InputFile<'a>) -> Self {
        Self {
            file,
            thumb: None,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a `Document` from bytes.
    pub fn bytes(filename: &'a str, bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            name: "document".into(),
            filename,
            bytes,
        })
    }

    /// Constructs a `Document` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: document's ID cannot start with `attach://`",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Document` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: document's URL cannot start with `attach://`",
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
}
