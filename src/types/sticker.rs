//! Types related to stickers.

use super::{
    file::{self, id::AsFileId},
    PhotoSize,
};
use serde::Deserialize;

pub mod mask_position;
mod set;

pub use {mask_position::MaskPosition, set::*};

/// Represents a [`Sticker`].
///
/// [`Sticker`]: https://core.telegram.org/bots/api#sticker
#[derive(Debug, PartialEq, Clone, Deserialize)]
// todo: #[non_exhaustive]
pub struct Sticker {
    /// The file ID of the sticker.
    pub file_id: file::Id,
    /// The width of the sticker.
    pub width: u32,
    /// The height of the sticker.
    pub height: u32,
    /// The thumb of the sticker.
    pub thumb: Option<PhotoSize>,
    /// The emoji of the sticker.
    pub emoji: Option<String>,
    /// The sticker set name which contains the sticker.
    pub set_name: Option<String>,
    /// The position of the sticker if it's a mask.
    pub mask_position: Option<MaskPosition>,
    /// The file size of the sticker.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for Sticker {}

impl AsFileId for Sticker {
    fn as_file_id(&self) -> file::id::Ref<'_> {
        self.file_id.as_ref()
    }
}
