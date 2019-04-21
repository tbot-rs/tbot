use super::*;

/// Represents a [`Voice`].
///
/// [`Voice`]: https://core.telegram.org/bots/api#voice
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Voice {
    /// The file ID of the voice.
    pub file_id: String,
    /// The duration of the voice.
    pub duration: u32,
    /// The MIME type of the voice.
    pub mime_type: Option<String>,
    /// The voice file's size.
    pub file_size: Option<u32>,
}
