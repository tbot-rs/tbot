use crate::types::file;
use serde::Deserialize;

/// Represents a [`Voice`].
///
/// [`Voice`]: https://core.telegram.org/bots/api#voice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Voice {
    /// The file ID of the voice.
    pub file_id: file::Id,
    /// The unique ID of the voice.
    pub file_unique_id: String,
    /// The duration of the voice.
    pub duration: u32,
    /// The MIME type of the voice.
    pub mime_type: Option<String>,
    /// The file size of the voice.
    pub file_size: Option<u32>,
}
