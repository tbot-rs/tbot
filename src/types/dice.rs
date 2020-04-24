//! Types related to dice.

use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::Serializer,
    Deserialize,
    Serialize,
};

/// Represents the kind of a thrown dice.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum Kind {
    /// 🎯.
    Darts,
    /// 🎲.
    Dice,
}

/// Represents a [`Dice`].
///
/// [`Dice`]: https://core.telegram.org/bots/api#dice
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
pub struct Dice {
    /// The value of the dice in the range [1, 6].
    pub value: u8,
    /// The kind of a thrown dice.
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
                    let _ = map.next_value::<serde_json::Value>();
                }
            }
        }

        let kind = match emoji.as_deref() {
            Some("🎯") => Kind::Darts,
            Some("🎲") => Kind::Dice,
            Some(unknown) => {
                return Err(de::Error::unknown_variant(unknown, &["🎯", "🎲"]))
            }
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
        })
    }
}
