use super::InputFile;
use serde::{Serialize, Serializer};
use std::borrow::Cow;

/// Represents a chat photo to be set.
///
/// Note that a chat photo cannot be set via either a file ID or a URL.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct ChatPhoto<'a>(pub(crate) InputFile<'a>);

impl<'a> ChatPhoto<'a> {
    /// Constructs a `ChatPhoto`.
    pub fn with_bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        let file = InputFile::File {
            filename: "photo.jpg".into(),
            bytes: bytes.into(),
        };

        ChatPhoto(file)
    }
}

impl<'a> Serialize for ChatPhoto<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer, "photo")
    }
}
