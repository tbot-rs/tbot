use super::*;

/// Represents an [`Animation`].
///
/// [`Animation`]: https://core.telegram.org/bots/api#animation
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Animation {
    /// The file ID of the animation.
    pub file_id: String,
    /// The width of the animation.
    pub width: u32,
    /// The height of the animation.
    pub height: u32,
    /// The duration of the animation.
    pub duration: u32,
    /// The thumb of the animation.
    pub thumb: Option<PhotoSize>,
    /// The MIME type of the animation.
    pub mime_type: Option<String>,
    /// The file size of the animation.
    pub file_size: Option<u32>,
}
