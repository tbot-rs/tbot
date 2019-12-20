//! Types related to callback queries.

use crate::types::{Message, User};

pub mod id;

pub use id::Id;

/// Represents the origin of the callback.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
#[must_use]
pub enum Origin {
    /// The callback comes from this message.
    Message(Box<Message>),
    /// The callback comes from an inline message with this ID.
    Inline(String),
}

/// Represents the kind of the callback.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// The callback is sent with some data.
    Data(String),
    /// The callback is sent to open a game.
    Game(String),
}

/// Represents a [`CallbackQuery`].
///
/// [`CallbackQuery`]: https://core.telegram.org/bots/api#callbackquery
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
#[must_use]
pub struct Query {
    /// The ID of the callback.
    pub id: Id,
    /// The user who initiated the callback.
    pub from: User,
    /// The origin of the query.
    pub origin: Origin,
    /// The identifier of the chat.
    pub chat_instance: String,
    /// The kind of the callback.
    pub kind: Kind,
}

impl Origin {
    /// Checks if `self` is `Message`.
    #[must_use]
    pub fn is_message(&self) -> bool {
        match self {
            Self::Message(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Inline`.
    #[must_use]
    pub fn is_inline(&self) -> bool {
        match self {
            Self::Inline(..) => true,
            _ => false,
        }
    }
}

impl Kind {
    /// Checks if `self` is `Data`.
    #[must_use]
    pub fn is_data(&self) -> bool {
        match self {
            Self::Data(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Game`.
    #[must_use]
    pub fn is_game(&self) -> bool {
        match self {
            Self::Game(..) => true,
            _ => false,
        }
    }
}

const ID: &str = "id";
const FROM: &str = "from";
const MESSAGE: &str = "message";
const INLINE_MESSAGE_ID: &str = "inline_message_id";
const CHAT_INSTANCE: &str = "chat_instance";
const DATA: &str = "data";
const GAME_SHORT_NAME: &str = "game_short_name";

struct QueryVisitor;

impl<'v> serde::de::Visitor<'v> for QueryVisitor {
    type Value = Query;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct Query")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut id = None;
        let mut from = None;
        let mut message = None;
        let mut inline_message_id = None;
        let mut chat_instance = None;
        let mut data = None;
        let mut game_short_name = None;

        while let Some(key) = map.next_key()? {
            match key {
                ID => id = Some(map.next_value()?),
                FROM => from = Some(map.next_value()?),
                MESSAGE => message = Some(map.next_value()?),
                INLINE_MESSAGE_ID => {
                    inline_message_id = Some(map.next_value()?)
                }
                CHAT_INSTANCE => chat_instance = Some(map.next_value()?),
                DATA => data = Some(map.next_value()?),
                GAME_SHORT_NAME => game_short_name = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let origin = if let Some(message) = message {
            Origin::Message(message)
        } else if let Some(inline_message_id) = inline_message_id {
            Origin::Inline(inline_message_id)
        } else {
            return Err(serde::de::Error::custom("Neither `message` nor `inline_message_id` was present on `CallbackQuery`"));
        };

        let kind = if let Some(data) = data {
            Kind::Data(data)
        } else if let Some(game_short_name) = game_short_name {
            Kind::Game(game_short_name)
        } else {
            return Err(serde::de::Error::custom("Neither `callback_data` nor `game_short_name` was present on `CallbackQuery`"));
        };

        Ok(Query {
            id: id.ok_or_else(|| serde::de::Error::missing_field(ID))?,
            from: from.ok_or_else(|| serde::de::Error::missing_field(FROM))?,
            origin,
            chat_instance: chat_instance.ok_or_else(|| {
                serde::de::Error::missing_field(CHAT_INSTANCE)
            })?,
            kind,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Query {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Query",
            &[
                ID,
                FROM,
                MESSAGE,
                INLINE_MESSAGE_ID,
                CHAT_INSTANCE,
                DATA,
                GAME_SHORT_NAME,
            ],
            QueryVisitor,
        )
    }
}
