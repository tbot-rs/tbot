use super::*;

/// Represents a [`StickerSet`].
///
/// [`StickerSet`]: https://core.telegram.org/bots/api#stickerset
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct StickerSet {
    /// The name of the sticker set (used in URLs).
    pub name: String,
    /// The title of the sticker set (shown to the user).
    pub title: String,
    /// `true` if the sticket set has masks.
    pub contains_masks: bool,
    /// The stickers from this set.
    pub stickers: Vec<Sticker>,
}
