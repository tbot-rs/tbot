use super::{InputFile, WithName};
use serde::Serialize;
use std::borrow::Cow;

/// Represents a chat photo to be set.
///
/// Note that a chat photo cannot be set via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct ChatPhoto<'a>(pub(crate) WithName<'a>);

impl<'a> ChatPhoto<'a> {
    /// Constructs a `ChatPhoto`.
    pub fn new(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        let file = InputFile::File {
            filename: "photo.jpg".into(),
            bytes: bytes.into(),
        };

        ChatPhoto(file.with_name("photo"))
    }
}
