//! Types representing inline keyboard buttons.

use crate::types::{callback::Game, LoginUrl};
use is_macro::Is;
use serde::{
    de::{self, Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor},
    ser::{Serialize, SerializeMap, Serializer},
};
use std::fmt::{self, Formatter};

/// Represents different types an inline button can be.
///
/// Complete descriptions can be found in [Bots API docs][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// Represents a URL button.
    Url(String),
    /// Represents a login button.
    LoginUrl(LoginUrl),
    /// Represents callback data.
    CallbackData(String),
    /// Represents query inserted when switched to inline.
    SwitchInlineQuery(String),
    /// Represents query inserted when switched to inline in the curent chat.
    SwitchInlineQueryCurrentChat(String),
    /// Represent a description of the game to be laucnhed.
    CallbackGame(Game),
    /// Represents a pay button.
    Pay,
}

impl Kind {
    /// Constructs a `ButtonKind::Url`.
    pub fn with_url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Constructs a `ButtonKind::LoginUrl`.
    pub const fn with_login_url(login_url: LoginUrl) -> Self {
        Self::LoginUrl(login_url)
    }

    /// Constructs a `ButtonKind::CallbackData`.
    pub fn with_callback_data(data: impl Into<String>) -> Self {
        Self::CallbackData(data.into())
    }

    /// Constructs a `ButtonKind::SwitchInlineQuery`.
    pub fn with_switch_inline_query(query: impl Into<String>) -> Self {
        Self::SwitchInlineQuery(query.into())
    }

    /// Constructs a `ButtonKind::SwitchInlineQueryCurrentChat`.
    pub fn with_switch_inline_query_current_chat(
        query: impl Into<String>,
    ) -> Self {
        Self::SwitchInlineQueryCurrentChat(query.into())
    }

    /// Constructs a `ButtonKind::CallbackGame`.
    pub const fn with_callback_game(game: Game) -> Self {
        Self::CallbackGame(game)
    }

    /// Constructs a `ButtonKind::Pay`.
    pub const fn with_pay() -> Self {
        Self::Pay
    }
}

/// Represents an [`InlineKeyboardButton`].
///
/// [`InlineKeyboardButton`]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Button {
    text: String,
    kind: Kind,
}

impl Button {
    /// Constructs an inline `Button`.
    pub fn new(text: impl Into<String>, kind: Kind) -> Self {
        Self {
            text: text.into(),
            kind,
        }
    }
}

impl Serialize for Button {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;

        map.serialize_entry("text", &self.text)?;

        match &self.kind {
            Kind::Url(url) => map.serialize_entry("url", url),
            Kind::LoginUrl(login_url) => {
                map.serialize_entry("login_url", login_url)
            }
            Kind::CallbackData(callback_data) => {
                map.serialize_entry("callback_data", callback_data)
            }
            Kind::SwitchInlineQuery(query) => {
                map.serialize_entry("switch_inline_query", query)
            }
            Kind::SwitchInlineQueryCurrentChat(query) => {
                map.serialize_entry("switch_inline_query_current_chat", query)
            }
            Kind::CallbackGame(game) => {
                map.serialize_entry("callback_game", game)
            }
            Kind::Pay => map.serialize_entry("pay", &true),
        }?;

        map.end()
    }
}

const TEXT: &str = "text";
const URL: &str = "url";
const CALLBACK_DATA: &str = "callback_data";
const SWITCH_INLINE_QUERY: &str = "switch_inline_query";
const SWITCH_INLINE_QUERY_CURRENT_CHAT: &str =
    "switch_inline_query_current_chat";
const CALLBACK_GAME: &str = "callback_game";
const PAY: &str = "pay";

struct ButtonVisitor;

impl<'v> Visitor<'v> for ButtonVisitor {
    type Value = Button;

    fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "struct Button")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        let mut text = None;
        let mut url = None;
        let mut callback_data = None;
        let mut switch_inline_query = None;
        let mut switch_inline_query_current_chat = None;
        let mut callback_game = None;
        let mut pay = None;

        while let Some(key) = map.next_key()? {
            match key {
                TEXT => text = Some(map.next_value()?),
                URL => url = Some(map.next_value()?),
                CALLBACK_DATA => callback_data = Some(map.next_value()?),
                SWITCH_INLINE_QUERY => {
                    switch_inline_query = Some(map.next_value()?)
                }
                SWITCH_INLINE_QUERY_CURRENT_CHAT => {
                    switch_inline_query_current_chat = Some(map.next_value()?)
                }
                CALLBACK_GAME => callback_game = Some(map.next_value()?),
                PAY => pay = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let kind = if let Some(url) = url {
            Kind::Url(url)
        } else if let Some(callback_data) = callback_data {
            Kind::CallbackData(callback_data)
        } else if let Some(callback_game) = callback_game {
            Kind::CallbackGame(callback_game)
        } else if let Some(switch_inline_query) = switch_inline_query {
            Kind::SwitchInlineQuery(switch_inline_query)
        } else if let Some(switch_inline_query_current_chat) =
            switch_inline_query_current_chat
        {
            Kind::SwitchInlineQueryCurrentChat(switch_inline_query_current_chat)
        } else if pay.is_some() {
            Kind::Pay
        } else {
            return Err(de::Error::custom("Could not construct Button's kind"));
        };

        Ok(Button {
            text: text.ok_or_else(|| de::Error::missing_field(TEXT))?,
            kind,
        })
    }
}

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Button",
            &[
                TEXT,
                URL,
                CALLBACK_DATA,
                SWITCH_INLINE_QUERY,
                SWITCH_INLINE_QUERY_CURRENT_CHAT,
                CALLBACK_GAME,
                PAY,
            ],
            ButtonVisitor,
        )
    }
}
