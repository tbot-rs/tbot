use super::InputFile;
use serde::Serialize;

/// Represents a chat photo to be set.
///
/// Note that a chat photo cannot be set via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct ChatPhoto<'a>(pub(crate) InputFile<'a>);

impl<'a> ChatPhoto<'a> {
    /// Constructs a `ChatPhoto`.
    pub fn new(bytes: &'a [u8]) -> Self {
        ChatPhoto(InputFile::File {
            name: "photo".into(),
            filename: "photo.jpg",
            bytes,
        })
    }
}
