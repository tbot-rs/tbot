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

pub(crate) enum Any {
    Sticker(Sticker),
    Animated(Animated),
}

const FILE_ID: &str = "file_id";
const WIDTH: &str = "width";
const HEIGHT: &str = "height";
const IS_ANIMATED: &str = "is_animated";
const THUMB: &str = "thumb";
const EMOJI: &str = "emoji";
const SET_NAME: &str = "set_name";
const MASK_POSITION: &str = "mask_position";
const FILE_SIZE: &str = "file_size";

struct AnyVisitor;

impl<'v> serde::de::Visitor<'v> for AnyVisitor {
    type Value = Any;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct Sticker")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut file_id = None;
        let mut width = None;
        let mut height = None;
        let mut is_animated = None;
        let mut thumb = None;
        let mut emoji = None;
        let mut set_name = None;
        let mut mask_position = None;
        let mut file_size = None;

        while let Some(key) = map.next_key()? {
            match key {
                FILE_ID => file_id = Some(map.next_value()?),
                WIDTH => width = Some(map.next_value()?),
                HEIGHT => height = Some(map.next_value()?),
                IS_ANIMATED => is_animated = Some(map.next_value()?),
                THUMB => thumb = Some(map.next_value()?),
                EMOJI => emoji = Some(map.next_value()?),
                SET_NAME => set_name = Some(map.next_value()?),
                MASK_POSITION => mask_position = Some(map.next_value()?),
                FILE_SIZE => file_size = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<serde_json::Value>();
                }
            }
        }

        let file_id =
            file_id.ok_or_else(|| serde::de::Error::missing_field(FILE_ID))?;
        let width =
            width.ok_or_else(|| serde::de::Error::missing_field(WIDTH))?;
        let height =
            height.ok_or_else(|| serde::de::Error::missing_field(HEIGHT))?;
        let is_animated = is_animated
            .ok_or_else(|| serde::de::Error::missing_field(IS_ANIMATED))?;

        let sticker = if is_animated {
            Any::Animated(Animated {
                file_id,
                width,
                height,
                thumb,
                emoji,
                set_name,
                file_size,
            })
        } else {
            Any::Sticker(Sticker {
                file_id,
                width,
                height,
                thumb,
                emoji,
                set_name,
                file_size,
                mask_position,
            })
        };

        Ok(sticker)
    }
}

impl<'de> serde::Deserialize<'de> for Any {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "Sticker",
            &[
                FILE_ID,
                WIDTH,
                HEIGHT,
                IS_ANIMATED,
                THUMB,
                EMOJI,
                SET_NAME,
                MASK_POSITION,
                FILE_SIZE,
            ],
            AnyVisitor,
        )
    }
}
