use super::*;

/// Represents a document to be sent.
pub struct Document<'a>(pub(crate) InputFile<'a>);

impl<'a> Document<'a> {
    /// Constructs a `Document` from bytes.
    pub fn file(filename: &'a str, bytes: &'a [u8]) -> Self {
        Document(InputFile::File {
            name: "Document".into(),
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
            "tbot: document's URL cannot start with `attach://`",
        );

        Document(InputFile::Id(id))
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

        Document(InputFile::Url(url))
    }
}
