use super::*;

/// Represents a [`Video`].
///
/// [`Video`]: https://core.telegram.org/bots/api#video
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Video {
    /// The file ID of the video.
    pub file_id: String,
    /// The width of the video.
    pub width: i64,
    /// The height of the video.
    pub height: i64,
    /// The duration of the video.
    pub duration: i64,
    /// The thumb of the video.
    pub thumb: Option<PhotoSize>,
    /// The MIME type of the video.
    pub mime_type: Option<String>,
    /// The video file's size.
    pub file_size: Option<i64>,
}
