use super::*;

/// Represents a [`VideoNote`].
///
/// [`VideoNote`]: https://core.telegram.org/bots/api#videonote
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct VideoNote {
    /// The file ID of the video note.
    pub file_id: String,
    /// The length of the video note.
    pub length: u32,
    /// The duration of the video note.
    pub duration: u32,
    /// The thumb ID of the video note.
    pub thumb: Option<PhotoSize>,
    /// The file size of the video note.
    pub file_size: Option<u32>,
}
