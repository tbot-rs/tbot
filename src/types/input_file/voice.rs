use super::*;

/// Represents a voice to be sent.
pub struct Voice<'a>(pub(crate) InputFile<'a>);

impl<'a> Voice<'a> {
    /// Constructs a `Voice` from bytes.
    pub fn file(bytes: &'a [u8]) -> Self {
        Voice(InputFile::File {
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
            "tbot: voice's ID cannot start with `attach://`",
        );

        Voice(InputFile::Id(id))
    }

    /// Constructs a `Voice` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "tbot: voice's URL cannot start with `attach://`",
        );

        Voice(InputFile::Url(url))
    }
}
