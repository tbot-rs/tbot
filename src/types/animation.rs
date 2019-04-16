use super::*;

/// Represents an [`Animation`].
///
/// [`Animation`]: https://core.telegram.org/bots/api#animation
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Animation {
    /// The file ID of the animation.
    pub file_id: String,
    /// The width of the animation.
    pub width: i64,
    /// The height of the animation.
    pub height: i64,
    /// The duration of the animation.
    pub duration: i64,
    /// The thumb of the animation.
    pub thumb: Option<PhotoSize>,
    /// The MIME type of the animation.
    pub mime_type: Option<String>,
    /// The animation file's size.
    pub file_size: Option<i64>,
}
