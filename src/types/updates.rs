use super::{callback, ChosenInlineResult, InlineQuery, Message, Poll};
use serde::de::{
    Deserialize, Deserializer, Error, IgnoredAny, MapAccess, Visitor,
};
use std::fmt::{self, Formatter};

/// Represents different types of updates from Telegram.
#[derive(Debug, PartialEq, Clone)]
// In fact, the large-sized variants are more common than the small-sized ones,
// so I think it's better not to box them.
#[allow(clippy::large_enum_variant)]
pub enum UpdateKind {
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
    /// A chosen inline result.
    ChosenInlineResult(ChosenInlineResult),
    /// Unknown update kind.
    Unknown,
}

/// Represents an update from Telegram.
#[derive(Debug)]
// todo: #[non_exhaustive]
pub struct Update {
    /// The ID of the update.
    pub id: u32,
    /// The kind of the update.
    pub kind: UpdateKind,
}

const UPDATE_ID: &str = "update_id";
const MESSAGE: &str = "message";
const EDITED_MESSAGE: &str = "edited_message";
const CHANNEL_POST: &str = "channel_post";
const EDITED_CHANNEL_POST: &str = "edited_channel_post";
const INLINE_QUERY: &str = "inline_query";
const CALLBACK_QUERY: &str = "callback_query";
const CHOSEN_INLINE_RESULT: &str = "chosen_inline_result";
const POLL: &str = "poll";

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateVisitor;

        impl<'v> Visitor<'v> for UpdateVisitor {
            type Value = Update;

            fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
                write!(fmt, "struct Update")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'v>,
            {
                let mut id = None;
                let mut kind = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        UPDATE_ID => id = Some(map.next_value()?),
                        MESSAGE => {
                            kind = Some(UpdateKind::Message(map.next_value()?))
                        }
                        EDITED_MESSAGE => {
                            kind = Some(UpdateKind::EditedMessage(
                                map.next_value()?,
                            ))
                        }
                        CHANNEL_POST => {
                            kind =
                                Some(UpdateKind::ChannelPost(map.next_value()?))
                        }
                        EDITED_CHANNEL_POST => {
                            kind = Some(UpdateKind::EditedChannelPost(
                                map.next_value()?,
                            ))
                        }
                        INLINE_QUERY => {
                            kind =
                                Some(UpdateKind::InlineQuery(map.next_value()?))
                        }
                        CALLBACK_QUERY => {
                            kind = Some(UpdateKind::CallbackQuery(
                                map.next_value()?,
                            ))
                        }
                        CHOSEN_INLINE_RESULT => {
                            kind = Some(UpdateKind::ChosenInlineResult(
                                map.next_value()?,
                            ))
                        }
                        POLL => {
                            kind = Some(UpdateKind::Poll(map.next_value()?))
                        }
                        _ => {
                            let _ = map.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                Ok(Update {
                    id: id.ok_or_else(|| Error::missing_field(UPDATE_ID))?,
                    kind: kind.unwrap_or(UpdateKind::Unknown),
                })
            }
        }

        deserializer.deserialize_struct(
            "Update",
            &[
                UPDATE_ID,
                MESSAGE,
                EDITED_MESSAGE,
                CHANNEL_POST,
                EDITED_CHANNEL_POST,
                INLINE_QUERY,
                CHOSEN_INLINE_RESULT,
            ],
            UpdateVisitor,
        )
    }
}
