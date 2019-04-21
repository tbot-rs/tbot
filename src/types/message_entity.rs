use super::*;

/// Represents an entity's kind.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum MessageEntityKind {
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

/// Represents a message's entity.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MessageEntity {
    /// The kind of the entity.
    pub kind: MessageEntityKind,
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

struct MessageEntityVisitor;

impl<'v> serde::de::Visitor<'v> for MessageEntityVisitor {
    type Value = MessageEntity;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct MessageEntity")
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
            TEXT_MENTION => MessageEntityKind::TextMention(
                user.ok_or_else(|| serde::de::Error::missing_field(USER))?,
            ),
            TEXT_LINK => MessageEntityKind::TextLink(
                url.ok_or_else(|| serde::de::Error::missing_field(URL))?,
            ),
            MENTION => MessageEntityKind::Mention,
            HASHTAG => MessageEntityKind::Hashtag,
            CASHTAG => MessageEntityKind::Cashtag,
            BOT_COMMAND => MessageEntityKind::BotCommand,
            URL => MessageEntityKind::Url,
            EMAIL => MessageEntityKind::Email,
            PHONE_NUMBER => MessageEntityKind::PhoneNumber,
            BOLD => MessageEntityKind::Bold,
            ITALIC => MessageEntityKind::Italic,
            CODE => MessageEntityKind::Code,
            PRE => MessageEntityKind::Pre,
            _ => {
                return Err(serde::de::Error::unknown_variant(
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

        Ok(MessageEntity {
            kind,
            offset: offset
                .ok_or_else(|| serde::de::Error::missing_field(OFFSET))?,
            length: length
                .ok_or_else(|| serde::de::Error::missing_field(LENGTH))?,
        })
    }
}

impl<'de> serde::Deserialize<'de> for MessageEntity {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "MessageEntity",
            &[TYPE, OFFSET, LENGTH, URL, USER],
            MessageEntityVisitor,
        )
    }
}
