//! Types related to dice.
#![allow(clippy::non_ascii_literal)]

use is_macro::Is;
use serde::{
    de::{self, Deserializer, IgnoredAny, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};

/// Represents the kind of a thrown dice.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
pub enum Kind {
    /// 🎯
    Darts,
    /// 🎲
    Dice,
    /// 🏀
    Basketball,
    /// Some emoji `tbot` isn't aware of yet.
    ///
    /// Please note that this field exists only to prevent parsing errors caused
    /// by unknown dice kinds, it is **not** meant to be matched on
    /// or constructed unless as a _temporary_ workaround until a new version
    /// of `tbot` with the new dice kind is released. In other words, we reserve
    /// the right to add new kinds to this enum and release them in patch
    /// updates, and we won't consider any breakage caused by this as a bug.
    /// You should also not construct this variant with an emoji covered by the
    /// above variants.
    Unknown(String),
}

/// Represents a [`Dice`].
///
/// [`Dice`]: https://core.telegram.org/bots/api#dice
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub struct Dice {
    /// The value of the dice in the range [1, 6].
    pub value: u8,
    /// The kind of the thrown dice.
    pub kind: Kind,
}

const VALUE: &str = "value";
const EMOJI: &str = "emoji";

struct DiceVisitor;

impl<'v> Visitor<'v> for DiceVisitor {
    type Value = Dice;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct Dice")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut value = None;
        let mut emoji: Option<String> = None;

        while let Some(key) = map.next_key()? {
            match key {
                VALUE => value = Some(map.next_value()?),
                EMOJI => emoji = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<IgnoredAny>();
                }
            }
        }

        let kind = match emoji.as_deref() {
            Some("🎯") => Kind::Darts,
            Some("🎲") => Kind::Dice,
            Some("🏀") => Kind::Basketball,
            Some(unknown) => Kind::Unknown(unknown.to_string()),
            None => return Err(de::Error::missing_field(EMOJI)),
        };

        Ok(Dice {
            kind,
            value: value.ok_or_else(|| de::Error::missing_field(VALUE))?,
        })
    }
}

impl<'de> Deserialize<'de> for Dice {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_struct("Dice", &[VALUE, EMOJI], DiceVisitor)
    }
}

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Dice => "🎲",
            Self::Darts => "🎯",
            Self::Basketball => "🏀",
            Self::Unknown(emoji) => emoji,
        })
    }
}
