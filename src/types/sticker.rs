//! Types related to stickers.

use super::{
    file::{self, id::AsFileId},
    PhotoSize,
};
use serde::Deserialize;

pub mod mask_position;
mod set;

pub use {mask_position::MaskPosition, set::*};

macro_rules! sticker_base {
    (
        $(#[doc = $struct_doc:literal])+
        struct $name:ident {
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)*
        }
    ) => {
        $(#[doc = $struct_doc])+
#[derive(Debug, PartialEq, Clone, Deserialize)]
// todo: #[non_exhaustive]
        pub struct $name {
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
    /// The file size of the sticker.
    pub file_size: Option<u32>,
            $(#[doc = $field_doc] $field: $type,)*
}

        impl crate::internal::Sealed for $name {}

        impl AsFileId for $name {
    fn as_file_id(&self) -> file::id::Ref<'_> {
        self.file_id.as_ref()
    }
}
    };
}

sticker_base! {
    /// Represents a [`Sticker`].
    ///
    /// [`Sticker`]: https://core.telegram.org/bots/api#sticker
    struct Sticker {
        /// The position of the sticker if it's a mask.
        mask_position: Option<MaskPosition>,
    }
}

sticker_base! {
    /// Represents an animated [`Sticker`].
    ///
    /// `tbot` chooses this struct when `Sticker.is_animated` is `true`.
    ///
    /// [`Sticker`]: https://core.telegram.org/bots/api#sticker
    struct Animated {}
}
