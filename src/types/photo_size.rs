use super::*;

/// Represents a [`PhotoSize`].
///
/// [`PhotoSize`]: https://core.telegram.org/bots/api#photosize
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct PhotoSize {
    /// The file ID of the photo.
    pub file_id: String,
    /// The width of the photo.
    pub width: u32,
    /// The height of the photo.
    pub height: u32,
    /// The file size of the photo.
    pub file_size: Option<u32>,
}
