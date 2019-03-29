use super::*;

/// Represents a [`StickerSet`].
///
/// [`SticketSet`]: https://core.telegram.org/bots/api#stickerset
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct StickerSet {
    /// The sticker set's name.
    pub name: String,
    /// The sticker set's title.
    pub title: String,
    /// Whehter the sticket set has masks.
    pub contains_masks: bool,
    /// The stickers of this set.
    pub stickers: Vec<Sticker>,
}
