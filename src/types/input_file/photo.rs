use super::*;

/// Represents a photo to be sent.
pub struct Photo<'a>(pub(crate) InputFile<'a>);

impl<'a> Photo<'a> {
    /// Constructs a `Photo` from bytes.
    pub fn file(bytes: &'a [u8]) -> Self {
        Photo(InputFile::File {
            name: "photo".into(),
            filename: "photo.jpg",
            bytes,
        })
    }

    /// Constructs a `Photo` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: photo's ID cannot start with `attach://`",
        );

        Photo(InputFile::Id(id))
    }

    /// Constructs a `Photo` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: photo's URL cannot start with `attach://`",
        );

        Photo(InputFile::Url(url))
    }
}
