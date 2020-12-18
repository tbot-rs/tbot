use crate::types::{file, PhotoSize};
use serde::Deserialize;

/// Represents an [`Audio`].
///
/// [`Audio`]: https://core.telegram.org/bots/api#audio
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Audio {
    /// The file ID of the audio.
    pub file_id: file::Id<'static>,
    /// The unique ID of the audio.
    pub file_unique_id: String,
    /// The duration of the audio.
    pub duration: u32,
    /// The performer of the audio.
    pub performer: Option<String>,
    /// The title of the audio.
    pub title: Option<String>,
    /// The original filename as defined by sender
    pub file_name: Option<String>,
    /// The MIME type of the audio.
    pub mime_type: Option<String>,
    /// The file size of the audio.
    pub file_size: Option<u32>,
    /// The thumb of the audio.
    pub thumb: Option<PhotoSize>,
}
