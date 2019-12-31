//! Types representing text.

use crate::types::User;
use is_macro::Is;
use serde::de::{Deserialize, Deserializer, Error, Visitor};
use std::fmt::{self, Formatter};

/// Represents either a text message or a caption.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Text {
    /// The text/caption. If there's no text, will be empty.
    pub value: String,
    /// The entities in the text/caption. If there are none, will be empty.
    pub entities: Vec<Entity>,
}

/// Represents an entity's kind.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
pub enum EntityKind {
    /// A mention.
    Mention,
    /// A hashtag.
    Hashtag,
    /// A cashtag (e.g. `$TBOT`).
    Cashtag,
    /// A bot command.
    BotCommand,
    /// An url.
    Url,
    /// An email.
    Email,
    /// A phone number.
    PhoneNumber,
    /// Text in bold.
    Bold,
    /// Text in italic.
    Italic,
    /// Underlined text.
    Underline,
    /// Strikethrough text.
    Strikethrough,
    /// String of monowidth text.
    Code,
    /// Block of monowidth text.
    Pre,
    /// A clickable text url.
    TextLink(String),
    /// A mention for users without username.
    TextMention(User),
}

/// Represents an entity of a message.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub struct Entity {
    /// The kind of the entity.
    pub kind: EntityKind,
    /// The offset at which the entity starts.
    pub offset: usize,
    /// The length of the entity.
    pub length: usize,
}

const OFFSET: &str = "offset";
const LENGTH: &str = "length";
const URL: &str = "url";
const USER: &str = "user";
const TYPE: &str = "type";

const MENTION: &str = "mention";
const HASHTAG: &str = "hashtag";
const CASHTAG: &str = "cashtag";
const BOT_COMMAND: &str = "bot_command";
// URL already defined
const EMAIL: &str = "email";
const PHONE_NUMBER: &str = "phone_number";
const BOLD: &str = "bold";
const ITALIC: &str = "italic";
const UNDERLINE: &str = "underline";
const STRIKETHROUGH: &str = "strikethrough";
const CODE: &str = "code";
const PRE: &str = "pre";
const TEXT_LINK: &str = "text_link";
const TEXT_MENTION: &str = "text_mention";

struct EntityVisitor;

impl<'v> Visitor<'v> for EntityVisitor {
    type Value = Entity;

    fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "struct Entity")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut kind: Option<String> = None;
        let mut offset = None;
        let mut length = None;
        let mut url = None;
        let mut user = None;

        while let Some(key) = map.next_key()? {
            match key {
                OFFSET => offset = Some(map.next_value()?),
                LENGTH => length = Some(map.next_value()?),
                URL => url = Some(map.next_value()?),
                USER => user = Some(map.next_value()?),
                TYPE => kind = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<serde_json::Value>();
                }
            }
        }

        let kind = kind.ok_or_else(|| serde::de::Error::missing_field(TYPE))?;

        let kind = match kind.as_str() {
            TEXT_MENTION => EntityKind::TextMention(
                user.ok_or_else(|| serde::de::Error::missing_field(USER))?,
            ),
            TEXT_LINK => EntityKind::TextLink(
                url.ok_or_else(|| serde::de::Error::missing_field(URL))?,
            ),
            MENTION => EntityKind::Mention,
            HASHTAG => EntityKind::Hashtag,
            CASHTAG => EntityKind::Cashtag,
            BOT_COMMAND => EntityKind::BotCommand,
            URL => EntityKind::Url,
            EMAIL => EntityKind::Email,
            PHONE_NUMBER => EntityKind::PhoneNumber,
            BOLD => EntityKind::Bold,
            ITALIC => EntityKind::Italic,
            UNDERLINE => EntityKind::Underline,
            STRIKETHROUGH => EntityKind::Strikethrough,
            CODE => EntityKind::Code,
            PRE => EntityKind::Pre,
            _ => {
                return Err(Error::unknown_variant(
                    &kind,
                    &[
                        MENTION,
                        HASHTAG,
                        CASHTAG,
                        BOT_COMMAND,
                        URL,
                        EMAIL,
                        PHONE_NUMBER,
                        BOLD,
                        ITALIC,
                        UNDERLINE,
                        STRIKETHROUGH,
                        CODE,
                        PRE,
                        TEXT_LINK,
                        TEXT_MENTION,
                    ],
                ))
            }
        };

        Ok(Entity {
            kind,
            offset: offset.ok_or_else(|| Error::missing_field(OFFSET))?,
            length: length.ok_or_else(|| Error::missing_field(LENGTH))?,
        })
    }
}

impl<'de> Deserialize<'de> for Entity {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_struct(
            "Entity",
            &[TYPE, OFFSET, LENGTH, URL, USER],
            EntityVisitor,
        )
    }
}
