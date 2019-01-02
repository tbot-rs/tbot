use super::*;

/// Represents a video note to be sent.
pub struct VideoNote<'a>(pub(crate) InputFile<'a>);

impl<'a> VideoNote<'a> {
    /// Constructs an `VideoNote` from bytes.
    pub fn file(bytes: &'a [u8]) -> Self {
        VideoNote(InputFile::File {
            name: "video_note".into(),
            filename: "video_note.mp4",
            bytes,
        })
    }

    /// Constructs a `VideoNote` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: video note's ID cannot start with `attach://`",
        );

        VideoNote(InputFile::Id(id))
    }

    /// Constructs a `VideoNote` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: video note's URL cannot start with `attach://`",
        );

        VideoNote(InputFile::Url(url))
    }
}
