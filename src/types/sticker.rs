//! Types related to stickers.

use super::{
    file::{self, id::AsFileId},
    PhotoSize,
};
use is_macro::Is;
use serde::de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};
use std::fmt::{self, Formatter};

pub mod mask_position;
pub mod set;

pub use {mask_position::MaskPosition, set::Set};

/// Represents different kinds of a [`Sticker`].
///
/// [`Sticker`]: ./struct.Sticker.html
#[derive(Debug, PartialEq, Clone, Copy, Is)]
#[non_exhaustive]
pub enum Kind {
    /// The sticker is neither animated nor a mask.
    Plain,
    /// The sticker is animated.
    Animated,
    /// The sticker is a mask.
    Mask(MaskPosition),
}

/// Represents a [`Sticker`].
///
/// [`Sticker`]: https://core.telegram.org/bots/api#sticker
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Sticker {
    /// The file ID of the sticker.
    pub file_id: file::Id<'static>,
    /// The unique ID of the sticker.
    pub file_unique_id: String,
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
    /// The kind of the sticker.
    pub kind: Kind,
}

impl crate::internal::Sealed for Sticker {}

impl AsFileId<'_> for Sticker {
    #[must_use]
    fn as_file_id(&self) -> file::id::Id<'_> {
        self.file_id.as_borrowed()
    }
}

const FILE_ID: &str = "file_id";
const FILE_UNIQUE_ID: &str = "file_unique_id";
const WIDTH: &str = "width";
const HEIGHT: &str = "height";
const IS_ANIMATED: &str = "is_animated";
const THUMB: &str = "thumb";
const EMOJI: &str = "emoji";
const SET_NAME: &str = "set_name";
const MASK_POSITION: &str = "mask_position";
const FILE_SIZE: &str = "file_size";

struct StickerVisitor;

impl<'v> Visitor<'v> for StickerVisitor {
    type Value = Sticker;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "struct Sticker")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut file_id = None;
        let mut file_unique_id = None;
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
                FILE_UNIQUE_ID => file_unique_id = Some(map.next_value()?),
                WIDTH => width = Some(map.next_value()?),
                HEIGHT => height = Some(map.next_value()?),
                IS_ANIMATED => is_animated = Some(map.next_value()?),
                THUMB => thumb = Some(map.next_value()?),
                EMOJI => emoji = Some(map.next_value()?),
                SET_NAME => set_name = Some(map.next_value()?),
                MASK_POSITION => mask_position = Some(map.next_value()?),
                FILE_SIZE => file_size = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let kind = if let Some(mask_position) = mask_position {
            Kind::Mask(mask_position)
        } else if is_animated == Some(true) {
            Kind::Animated
        } else {
            Kind::Plain
        };

        Ok(Sticker {
            file_id: file_id
                .ok_or_else(|| serde::de::Error::missing_field(FILE_ID))?,
            file_unique_id: file_unique_id.ok_or_else(|| {
                serde::de::Error::missing_field(FILE_UNIQUE_ID)
            })?,
            width: width
                .ok_or_else(|| serde::de::Error::missing_field(WIDTH))?,
            height: height
                .ok_or_else(|| serde::de::Error::missing_field(HEIGHT))?,
            thumb,
            emoji,
            set_name,
            file_size,
            kind,
        })
    }
}

impl<'de> Deserialize<'de> for Sticker {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_struct(
            "Sticker",
            &[
                FILE_ID,
                FILE_UNIQUE_ID,
                WIDTH,
                HEIGHT,
                IS_ANIMATED,
                THUMB,
                EMOJI,
                SET_NAME,
                MASK_POSITION,
                FILE_SIZE,
            ],
            StickerVisitor,
        )
    }
}
