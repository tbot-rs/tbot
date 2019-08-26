//! Types related to updates.

use super::{
    callback, shipping, ChosenInlineResult, InlineQuery, Message, Poll,
    PreCheckoutQuery,
};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{self, Formatter};

/// Represents an update ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(pub isize);

/// Represents different types of updates from Telegram.
#[derive(Debug, PartialEq, Clone)]
// In fact, the large-sized variants are more common than the small-sized ones,
// so I think it's better not to box them.
#[allow(clippy::large_enum_variant)]
// todo: #[non_exhaustive]
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
// todo: #[non_exhaustive]
pub struct Update {
    /// The ID of the update.
    pub id: Id,
    /// The kind of the update.
    pub kind: Kind,
}

impl Kind {
    /// Checks if `self` is `Message`.
    pub fn is_message(&self) -> bool {
        match self {
            Kind::Message(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `EditedMessage`.
    pub fn is_edited_message(&self) -> bool {
        match self {
            Kind::EditedMessage(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `ChannelPost`.
    pub fn is_channel_post(&self) -> bool {
        match self {
            Kind::ChannelPost(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `EditedChannelPost`.
    pub fn is_edited_channel_post(&self) -> bool {
        match self {
            Kind::EditedChannelPost(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `InlineQuery`.
    pub fn is_inline_query(&self) -> bool {
        match self {
            Kind::InlineQuery(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `CallbackQuery`.
    pub fn is_callback_query(&self) -> bool {
        match self {
            Kind::CallbackQuery(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Poll`.
    pub fn is_poll(&self) -> bool {
        match self {
            Kind::Poll(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `ChosenInlineResult`.
    pub fn is_chosen_inline_result(&self) -> bool {
        match self {
            Kind::ChosenInlineResult(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `ShippingQuery`.
    pub fn is_shipping_query(&self) -> bool {
        match self {
            Kind::ShippingQuery(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `PreCheckoutQuery`.
    pub fn is_pre_checkout_query(&self) -> bool {
        match self {
            Kind::PreCheckoutQuery(..) => true,
            _ => false,
        }
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
                MESSAGE => kind = Some(Kind::Message(map.next_value()?)),
                EDITED_MESSAGE => {
                    kind = Some(Kind::EditedMessage(map.next_value()?))
                }
                CHANNEL_POST => {
                    kind = Some(Kind::ChannelPost(map.next_value()?))
                }
                EDITED_CHANNEL_POST => {
                    kind = Some(Kind::EditedChannelPost(map.next_value()?))
                }
                INLINE_QUERY => {
                    kind = Some(Kind::InlineQuery(map.next_value()?))
                }
                CALLBACK_QUERY => {
                    kind = Some(Kind::CallbackQuery(map.next_value()?))
                }
                CHOSEN_INLINE_RESULT => {
                    kind = Some(Kind::ChosenInlineResult(map.next_value()?))
                }
                SHIPPING_QUERY => {
                    kind = Some(Kind::ShippingQuery(map.next_value()?))
                }
                PRE_CHECKOUT_QUERY => {
                    kind = Some(Kind::PreCheckoutQuery(map.next_value()?))
                }
                POLL => kind = Some(Kind::Poll(map.next_value()?)),
                _ => {
                    let _ = map.next_value::<IgnoredAny>()?;
                }
            }
        }

        Ok(Update {
            id: id.ok_or_else(|| Error::missing_field(UPDATE_ID))?,
            kind: kind.unwrap_or(Kind::Unknown),
        })
    }
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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
                SHIPPING_QUERY,
                PRE_CHECKOUT_QUERY,
            ],
            UpdateVisitor,
        )
    }
}
