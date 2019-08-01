use super::{InputFile, WithName};
use crate::types::value::Bytes;
use serde::Serialize;

/// Represents a chat photo to be set.
///
/// Note that a chat photo cannot be set via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct ChatPhoto<'a>(pub(crate) WithName<'a>);

impl<'a> ChatPhoto<'a> {
    /// Constructs a `ChatPhoto`.
    pub fn new(bytes: impl Into<Bytes<'a>>) -> Self {
        let file = InputFile::File {
            filename: "photo.jpg".into(),
            bytes: bytes.into(),
        };

        ChatPhoto(file.own_with_name("photo"))
    }
}
