//! Types related to sticker sets.

use super::Sticker;
use crate::types::PhotoSize;
use is_macro::Is;
use serde::de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};
use std::fmt::{self, Formatter};

/// Represents different kinds of a [`sticker::Set`].
///
/// [`sticker::Set`]: Set
#[derive(Debug, PartialEq, Clone, Copy, Is)]
#[non_exhaustive]
pub enum Kind {
    /// The stickers in the sticker set are neither animated nor masks.
    Plain,
    /// The sticker set contains animated stickers.
    Animated,
    /// The sticker set contains masks.
    Masks,
}

/// Represents a [`StickerSet`].
///
/// [`StickerSet`]: https://core.telegram.org/bots/api#stickerset
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Set {
    /// The name of the sticker set (used in URLs).
    pub name: String,
    /// The title of the sticker set (shown to the user).
    pub title: String,
    /// The kind of the sticker set.
    pub kind: Kind,
    /// The stickers from this set.
    pub stickers: Vec<Sticker>,
    /// The thumb of the sticker set.
    pub thumb: Option<PhotoSize>,
}

const NAME: &str = "name";
const TITLE: &str = "title";
const IS_ANIMATED: &str = "is_animated";
const CONTAINS_MASKS: &str = "contains_masks";
const STICKERS: &str = "stickers";
const THUMB: &str = "thumb";

struct SetVisitor;

impl<'v> Visitor<'v> for SetVisitor {
    type Value = Set;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "struct sticker::Set")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut name = None;
        let mut title = None;
        let mut is_animated = None;
        let mut contains_masks = None;
        let mut stickers = None;
        let mut thumb = None;

        while let Some(key) = map.next_key()? {
            match key {
                NAME => name = Some(map.next_value()?),
                TITLE => title = Some(map.next_value()?),
                IS_ANIMATED => is_animated = Some(map.next_value()?),
                CONTAINS_MASKS => contains_masks = Some(map.next_value()?),
                STICKERS => stickers = Some(map.next_value()?),
                THUMB => thumb = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let kind = if contains_masks == Some(true) {
            Kind::Masks
        } else if is_animated == Some(true) {
            Kind::Animated
        } else {
            Kind::Plain
        };

        Ok(Set {
            name: name.ok_or_else(|| serde::de::Error::missing_field(NAME))?,
            title: title
                .ok_or_else(|| serde::de::Error::missing_field(TITLE))?,
            stickers: stickers
                .ok_or_else(|| serde::de::Error::missing_field(STICKERS))?,
            thumb,
            kind,
        })
    }
}

impl<'de> Deserialize<'de> for Set {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "sticker::Set",
            &[NAME, TITLE, IS_ANIMATED, CONTAINS_MASKS, STICKERS, THUMB],
            SetVisitor,
        )
    }
}
