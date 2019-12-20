//! Types representing text.

use crate::types::User;
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
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    /// String of monowidth text.
    Code,
    /// Block of monowidth text.
    Pre,
    /// A clickable text url.
    TextLink(String),
    /// A mention for users without username.
    TextMention(User),
}

impl EntityKind {
    /// Checks if `self` is `Mention`.
    #[must_use]
    pub fn is_mention(&self) -> bool {
        *self == Self::Mention
    }

    /// Checks if `self` is `Hashtag`.
    #[must_use]
    pub fn is_hastag(&self) -> bool {
        *self == Self::Hashtag
    }

    /// Checks if `self` is `Cashtag`.
    #[must_use]
    pub fn is_cashtag(&self) -> bool {
        *self == Self::Cashtag
    }

    /// Checks if `self` is `BotCommand`.
    #[must_use]
    pub fn is_bot_command(&self) -> bool {
        *self == Self::BotCommand
    }

    /// Checks if `self` is `Url`.
    #[must_use]
    pub fn is_url(&self) -> bool {
        *self == Self::Url
    }

    /// Checks if `self` is `Email`.
    #[must_use]
    pub fn is_email(&self) -> bool {
        *self == Self::Email
    }

    /// Checks if `self` is `PhoneNumber`.
    #[must_use]
    pub fn is_phone_number(&self) -> bool {
        *self == Self::PhoneNumber
    }

    /// Checks if `self` is `Bold`.
    #[must_use]
    pub fn is_bold(&self) -> bool {
        *self == Self::Bold
    }

    /// Checks if `self` is `Italic`.
    #[must_use]
    pub fn is_italic(&self) -> bool {
        *self == Self::Italic
    }

    /// Checks if `self` is `Code`.
    #[must_use]
    pub fn is_code(&self) -> bool {
        *self == Self::Code
    }

    /// Checks if `self` is `Pre`.
    #[must_use]
    pub fn is_pre(&self) -> bool {
        *self == Self::Pre
    }

    /// Checks if `self` is `TextLink`.
    #[must_use]
    pub fn is_text_link(&self) -> bool {
        match self {
            Self::TextLink(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `TextMention`.
    #[must_use]
    pub fn is_text_mention(&self) -> bool {
        match self {
            Self::TextMention(..) => true,
            _ => false,
        }
    }
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
