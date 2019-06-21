use super::*;
use crate::types::file::id::AsFileId;

/// Represents a [`Voice`].
///
/// [`Voice`]: https://core.telegram.org/bots/api#voice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Voice {
    /// The file ID of the voice.
    pub file_id: file::Id,
    /// The duration of the voice.
    pub duration: u32,
    /// The MIME type of the voice.
    pub mime_type: Option<String>,
    /// The file size of the voice.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for Voice {}

impl AsFileId for Voice {
    fn as_file_id(&self) -> file::id::Ref<'_> {
        self.file_id.as_ref()
    }
}
