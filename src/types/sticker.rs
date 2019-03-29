use super::*;

/// Represents a [`Sticker`].
///
/// [`Sticker`]: https://core.telegram.org/bots/api#sticker
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Sticker {
    /// The file ID of the sticker.
    pub file_id: String,
    /// The width of the sticker.
    pub width: i64,
    /// The height of the sticker.
    pub height: i64,
    /// The thumb of the sticker.
    pub thumb: Option<PhotoSize>,
    /// The emoji of the sticker.
    pub emoji: Option<String>,
    /// The sticker set name with the sticker.
    pub set_name: Option<String>,
    /// The sticket's position if it is a mask.
    pub mask_position: Option<MaskPosition>,
    /// The sticker file's size.
    pub file_size: Option<i64>,
}
