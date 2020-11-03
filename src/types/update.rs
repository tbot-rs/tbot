//! Types related to updates.

use super::{
    callback, poll::Answer, shipping, ChosenInlineResult, InlineQuery, Message,
    Poll, PreCheckoutQuery,
};
use is_macro::Is;
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
    Deserialize,
};
use std::{
    convert::TryFrom,
    fmt::{self, Formatter},
};

/// Represents an update ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(pub isize);

/// Represents different types of updates from Telegram.
#[derive(Debug, PartialEq, Clone, Is)]
// In fact, the large-sized variants are more common than the small-sized ones,
// so I think it's better not to box them.
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Kind {
    /// A new chat message.
    Message(Message),
    /// An edited message.
    EditedMessage(Message),
    /// A new channel post.
    ChannelPost(Message),
    /// An edited channel post.
    EditedChannelPost(Message),
    /// An inline query.
    InlineQuery(InlineQuery),
    /// An incoming callback query.
    CallbackQuery(callback::Query),
    /// A new state of a poll.
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll.
    PollAnswer(Answer),
    /// A chosen inline result.
    ChosenInlineResult(ChosenInlineResult),
    /// A shipping query.
    ShippingQuery(shipping::Query),
    /// A pre-checkout query.
    PreCheckoutQuery(PreCheckoutQuery),
    /// Unknown update kind.
    Unknown,
}

/// Represents an update from Telegram.
#[derive(Debug)]
#[non_exhaustive]
pub struct Update {
    /// The ID of the update.
    pub id: Id,
    /// The kind of the update.
    pub kind: Kind,
}

#[derive(Debug)]
pub(crate) struct RawUpdate {
    pub id: Id,
    pub kind: Result<Kind, String>,
}

impl TryFrom<RawUpdate> for Update {
    type Error = String;

    fn try_from(raw_update: RawUpdate) -> Result<Self, Self::Error> {
        Ok(Self {
            id: raw_update.id,
            kind: raw_update.kind?,
        })
    }
}

const UPDATE_ID: &str = "update_id";
const MESSAGE: &str = "message";
const EDITED_MESSAGE: &str = "edited_message";
const CHANNEL_POST: &str = "channel_post";
const EDITED_CHANNEL_POST: &str = "edited_channel_post";
const INLINE_QUERY: &str = "inline_query";
const CALLBACK_QUERY: &str = "callback_query";
const CHOSEN_INLINE_RESULT: &str = "chosen_inline_result";
const SHIPPING_QUERY: &str = "shipping_query";
const PRE_CHECKOUT_QUERY: &str = "pre_checkout_query";
const POLL: &str = "poll";
const POLL_ANSWER: &str = "poll_answer";

struct RawUpdateVisitor;

impl<'v> Visitor<'v> for RawUpdateVisitor {
    type Value = RawUpdate;

    fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "struct RawUpdate")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut id = None;
        let mut kind = Ok(Kind::Unknown);

        while let Some(key) = map.next_key()? {
            kind = match key {
                UPDATE_ID => {
                    id = Some(map.next_value()?);
                    continue;
                }
                MESSAGE => map.next_value().map(Kind::Message),
                EDITED_MESSAGE => map.next_value().map(Kind::EditedMessage),
                CHANNEL_POST => map.next_value().map(Kind::ChannelPost),
                EDITED_CHANNEL_POST => {
                    map.next_value().map(Kind::EditedChannelPost)
                }
                INLINE_QUERY => map.next_value().map(Kind::InlineQuery),
                CALLBACK_QUERY => map.next_value().map(Kind::CallbackQuery),
                CHOSEN_INLINE_RESULT => {
                    map.next_value().map(Kind::ChosenInlineResult)
                }
                SHIPPING_QUERY => map.next_value().map(Kind::ShippingQuery),
                PRE_CHECKOUT_QUERY => {
                    map.next_value().map(Kind::PreCheckoutQuery)
                }
                POLL => map.next_value().map(Kind::Poll),
                POLL_ANSWER => map.next_value().map(Kind::PollAnswer),
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                    Ok(Kind::Unknown)
                }
            };
        }

        Ok(RawUpdate {
            id: id.ok_or_else(|| Error::missing_field(UPDATE_ID))?,
            kind: kind.map_err(|x| x.to_string()),
        })
    }
}

impl<'de> Deserialize<'de> for RawUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "RawUpdate",
            &[
                UPDATE_ID,
                MESSAGE,
                EDITED_MESSAGE,
                CHANNEL_POST,
                EDITED_CHANNEL_POST,
                INLINE_QUERY,
                CHOSEN_INLINE_RESULT,
                SHIPPING_QUERY,
                PRE_CHECKOUT_QUERY,
                POLL,
                POLL_ANSWER,
            ],
            RawUpdateVisitor,
        )
    }
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(RawUpdate::deserialize(deserializer)?)
            .map_err(Error::custom)
    }
}
