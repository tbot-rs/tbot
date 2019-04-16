use super::*;

/// Represents a [`VideoNote`].
///
/// [`VideoNote`]: https://core.telegram.org/bots/api#videonote
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct VideoNote {
    /// The file ID of the video note.
    pub file_id: String,
    /// The length of the video note.
    pub length: i64,
    /// The duration of the video note.
    pub duration: i64,
    /// The thumb ID of the video_note.
    pub thumb: Option<PhotoSize>,
    /// The video note file's size.
    pub file_size: Option<i64>,
}
