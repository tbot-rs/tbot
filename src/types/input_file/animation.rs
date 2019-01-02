use super::*;

/// Represents an animation to be sent.
pub struct Animation<'a>(pub(crate) InputFile<'a>);

impl<'a> Animation<'a> {
    /// Constructs an `Animation` from bytes.
    pub fn file(bytes: &'a [u8]) -> Self {
        Animation(InputFile::File {
            name: "animaion".into(),
            filename: "animation.mp4",
            bytes,
        })
    }

    /// Constructs an `Animation` from a file ID.
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: &'a str) -> Self {
        assert!(
            !id.starts_with("attach://"),
            "tbot: video note's URL cannot start with `attach://`",
        );

        Animation(InputFile::Id(id))
    }

    /// Constructs an `Animation` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: animation's URL cannot start with `attach://`",
        );

        Animation(InputFile::Url(url))
    }
}
