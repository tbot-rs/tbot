use super::*;

/// Represents a video to be sent.
pub struct Video<'a>(pub(crate) InputFile<'a>);

impl<'a> Video<'a> {
    /// Constructs a `Video` from bytes.
    pub fn file(bytes: &'a [u8]) -> Self {
        Video(InputFile::File {
            name: "video".into(),
            filename: "video.mp4",
            bytes,
        })
    }

    /// Constructs a `Video` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: video's ID cannot start with `attach://`",
        );

        Video(InputFile::Id(id))
    }

    /// Constructs a `Video` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: video's URL cannot start with `attach://`",
        );

        Video(InputFile::Url(url))
    }
}
