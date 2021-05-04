//! Types representing inline keyboard buttons.

use crate::types::{callback::Game, LoginUrl};
use is_macro::Is;
use serde::ser::{Serialize, SerializeMap, Serializer};

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
