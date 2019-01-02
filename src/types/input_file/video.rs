use super::*;

/// Represents an video to be sent.
pub struct Video<'a>(pub(crate) InputFile<'a>);

impl<'a> Video<'a> {
    /// Constructs an `Video` from bytes.
    pub fn file(bytes: &'a [u8]) -> Self {
        Video(InputFile::File {
            name: "animaion".into(),
            filename: "video.mp4",
            bytes,
        })
    }

    /// Constructs an `Video` from a file ID.
    pub fn id(id: &'a str) -> Self {
        Video(InputFile::Id(id))
    }

    /// Constructs an `Video` from an URL.
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
