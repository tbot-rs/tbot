use super::*;

/// Represents an [`Audio`].
///
/// [`Audio`]: https://core.telegram.org/bots/api#audio
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Audio {
    /// The file ID of the audio.
    pub file_id: String,
    /// The duration of the audio.
    pub duration: i64,
    /// The performer of the audio.
    pub performer: Option<String>,
    /// The title of the audio.
    pub title: Option<String>,
    /// The MIME type of the audio.
    pub mime_type: Option<String>,
    /// The audio file's size.
    pub file_size: Option<i64>,
    /// The thumb of the audio.
    pub thumb: Option<PhotoSize>,
}
