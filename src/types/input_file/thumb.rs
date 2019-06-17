use super::InputFile;
use serde::Serialize;

/// Represents a thumb to be sent.
///
/// Note that a thumb cannot be sent via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Thumb<'a>(pub(crate) InputFile<'a>);

impl<'a> Thumb<'a> {
    /// Constructs a `Thumb`.
    pub fn new(bytes: &'a [u8]) -> Self {
        Thumb(InputFile::File {
            name: "thumb".into(),
            filename: "thumb.jpg",
            bytes,
        })
    }
}
