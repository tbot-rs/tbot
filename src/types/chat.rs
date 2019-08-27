//! Types related to chats.

use super::*;

mod action;
mod id;
mod kind;
pub mod member;
mod permissions;
mod photo;

pub use {action::*, id::*, kind::*, member::*, permissions::*, photo::*};

/// Represents a [`Chat`].
///
/// [`Chat`]: https://core.telegram.org/bots/api#chat
#[derive(Debug, PartialEq, Clone)]
// todo: #[non_exhaustive]
pub struct Chat {
    /// The ID of the chat.
    pub id: Id,
    /// The kind of the chat.
    pub kind: Kind,
    /// The photo of the chat.
    pub photo: Option<Photo>,
}

const ID: &str = "id";
const KIND: &str = "type";
const TITLE: &str = "title";
const USERNAME: &str = "username";
const FIRST_NAME: &str = "first_name";
const LAST_NAME: &str = "last_name";
const PERMISSIONS: &str = "permissions";
const PHOTO: &str = "photo";
const DESCRIPTION: &str = "description";
const INIVITE_LINK: &str = "invite_link";
const PINNED_MESSAGE: &str = "pinned_message";
const STICKER_SET_NAME: &str = "sticker_set_name";
const CAN_SET_STICKER_SET: &str = "can_set_sticker_set";

const PRIVATE: &str = "private";
const GROUP: &str = "group";
const SUPERGROUP: &str = "supergroup";
const CHANNEL: &str = "channel";

struct ChatVisitor;

impl<'v> serde::de::Visitor<'v> for ChatVisitor {
    type Value = Chat;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct Chat")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut id = None;
        let mut kind = None;
        let mut title = None;
        let mut username = None;
        let mut first_name = None;
        let mut last_name = None;
        let mut permissions = None;
        let mut photo = None;
        let mut description = None;
        let mut invite_link = None;
        let mut pinned_message = None;
        let mut sticker_set_name = None;
        let mut can_set_sticker_set = None;

        while let Some(key) = map.next_key()? {
            match key {
                ID => id = Some(map.next_value()?),
                KIND => kind = Some(map.next_value()?),
                TITLE => title = Some(map.next_value()?),
                USERNAME => username = Some(map.next_value()?),
                FIRST_NAME => first_name = Some(map.next_value()?),
                LAST_NAME => last_name = Some(map.next_value()?),
                PERMISSIONS => permissions = Some(map.next_value()?),
                PHOTO => photo = Some(map.next_value()?),
                DESCRIPTION => description = Some(map.next_value()?),
                INIVITE_LINK => invite_link = Some(map.next_value()?),
                PINNED_MESSAGE => pinned_message = Some(map.next_value()?),
                STICKER_SET_NAME => sticker_set_name = Some(map.next_value()?),
                CAN_SET_STICKER_SET => {
                    can_set_sticker_set = Some(map.next_value()?)
                }
                _ => {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let kind = match &kind {
            Some(PRIVATE) => Kind::Private {
                username,
                first_name: first_name.ok_or_else(|| {
                    serde::de::Error::missing_field(FIRST_NAME)
                })?,
                last_name,
            },
            Some(GROUP) => Kind::Group {
                title: title
                    .ok_or_else(|| serde::de::Error::missing_field(TITLE))?,
                pinned_message,
                permissions,
            },
            Some(SUPERGROUP) => Kind::Supergroup {
                title: title
                    .ok_or_else(|| serde::de::Error::missing_field(TITLE))?,
                username,
                description,
                invite_link,
                pinned_message,
                sticker_set_name,
                can_set_sticker_set,
                permissions,
            },
            Some(CHANNEL) => Kind::Channel {
                title: title
                    .ok_or_else(|| serde::de::Error::missing_field(TITLE))?,
                username,
                description,
                invite_link,
                pinned_message,
            },
            None => return Err(serde::de::Error::missing_field(KIND)),
            Some(unknown_kind) => {
                return Err(serde::de::Error::unknown_variant(
                    unknown_kind,
                    &[PRIVATE, GROUP, SUPERGROUP, CHANNEL],
                ))
            }
        };

        Ok(Chat {
            id: id.ok_or_else(|| serde::de::Error::missing_field(ID))?,
            kind,
            photo,
        })
    }
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Chat",
            &[
                ID,
                KIND,
                TITLE,
                USERNAME,
                FIRST_NAME,
                LAST_NAME,
                PERMISSIONS,
                PHOTO,
                DESCRIPTION,
                INIVITE_LINK,
                PINNED_MESSAGE,
                STICKER_SET_NAME,
                CAN_SET_STICKER_SET,
            ],
            ChatVisitor,
        )
    }
}
