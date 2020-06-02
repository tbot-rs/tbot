use crate::types::file::{self, id::AsFileId};
use serde::Deserialize;

/// Represents a [`Voice`].
///
/// [`Voice`]: https://core.telegram.org/bots/api#voice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Voice {
    /// The file ID of the voice.
    pub file_id: file::Id<'static>,
    /// The unique ID of the voice.
    pub file_unique_id: String,
    /// The duration of the voice.
    pub duration: u32,
    /// The MIME type of the voice.
    pub mime_type: Option<String>,
    /// The file size of the voice.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for Voice {}

impl<'a> AsFileId<'a> for Voice {
    #[must_use]
    fn as_file_id(&self) -> file::id::Id<'_> {
        self.file_id.as_borrowed()
    }
}
