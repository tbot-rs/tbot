use super::{InputFile, WithName};
use serde::Serialize;

/// Represents a chat photo to be set.
///
/// Note that a chat photo cannot be set via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct ChatPhoto<'a>(pub(crate) WithName<'a>);

impl<'a> ChatPhoto<'a> {
    /// Constructs a `ChatPhoto`.
    pub fn new(bytes: &'a [u8]) -> Self {
        let file = InputFile::File {
            filename: "photo.jpg",
            bytes,
        };

        ChatPhoto(file.with_name("photo"))
    }
}
