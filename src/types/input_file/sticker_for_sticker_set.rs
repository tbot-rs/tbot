use super::{PngSticker, TgsSticker};
use is_macro::Is;

/// Represents a sticker that can be added to a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
pub enum StickerForStickerSet {
    /// A PNG sticker.
    Png(PngSticker),
    /// A TGS (animated) sticker.
    Tgs(TgsSticker),
}

impl From<PngSticker> for StickerForStickerSet {
    fn from(sticker: PngSticker) -> Self {
        Self::Png(sticker)
    }
}

impl From<TgsSticker> for StickerForStickerSet {
    fn from(sticker: TgsSticker) -> Self {
        Self::Tgs(sticker)
    }
}
