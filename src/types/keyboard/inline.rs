//! Types representing inline keyboards.

use crate::types::{callback::Game, LoginUrl};
use serde::{ser::SerializeMap, Serialize};

/// A shorthand for inline markup.
pub type Markup<'a> = &'a [&'a [Button<'a>]];

/// Represents different types an inline button can be.
///
/// Complete descriptions can be found in [Bots API docs][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum ButtonKind<'a> {
    /// Represents a URL button.
    Url(&'a str),
    /// Represents a login button.
    LoginUrl(LoginUrl<'a>),
    /// Represents callback data.
    CallbackData(&'a str),
    /// Represents query inserted when switched to inline.
    SwitchInlineQuery(&'a str),
    /// Represents query inserted when switched to inline in the curent chat.
    SwitchInlineQueryCurrentChat(&'a str),
    /// Represent a description of the game to be laucnhed.
    CallbackGame(Game),
    /// If `true`, a pay button is sent.
    Pay(bool),
}

impl ButtonKind<'_> {
    /// Checks if `self` is `Url`.
    pub fn is_url(&self) -> bool {
        match self {
            ButtonKind::Url(..) => true,
            _ => false,
        }
    }
    /// Checks if `self` is `LoginUrl`.
    pub fn is_login_url(&self) -> bool {
        match self {
            ButtonKind::LoginUrl(..) => true,
            _ => false,
        }
    }
    /// Checks if `self` is `CallbackData`.
    pub fn is_callback_data(&self) -> bool {
        match self {
            ButtonKind::CallbackData(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SwitchInlineQuery`.
    pub fn is_switch_inline_query(&self) -> bool {
        match self {
            ButtonKind::SwitchInlineQuery(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SwitchInlineQueryCurrentChat`.
    pub fn is_switch_inline_query_current_chat(&self) -> bool {
        // what a name

        match self {
            ButtonKind::SwitchInlineQueryCurrentChat(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `CallbackGame`.
    pub fn is_callback_game(&self) -> bool {
        match self {
            ButtonKind::CallbackGame(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Pay`.
    pub fn is_pay(&self) -> bool {
        match self {
            ButtonKind::Pay(..) => true,
            _ => false,
        }
    }
}

/// Represents an [`InlineKeyboardButton`].
///
/// [`InlineKeyboardButton`]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct Button<'a> {
    text: &'a str,
    kind: ButtonKind<'a>,
}

/// Represents an [`InlineKeyboardMarkup`].
///
/// [`InlineKeyboardMarkup`]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Keyboard<'a> {
    inline_keyboard: Markup<'a>,
}

impl<'a> Button<'a> {
    /// Constructs an inline `Button`.
    pub const fn new(text: &'a str, kind: ButtonKind<'a>) -> Self {
        Self { text, kind }
    }
}

impl Serialize for Button<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(Some(2))?;

        map.serialize_entry("text", self.text)?;

        match self.kind {
            ButtonKind::Url(url) => map.serialize_entry("url", url),
            ButtonKind::LoginUrl(login_url) => {
                map.serialize_entry("login_url", &login_url)
            }
            ButtonKind::CallbackData(callback_data) => {
                map.serialize_entry("callback_data", callback_data)
            }
            ButtonKind::SwitchInlineQuery(query) => {
                map.serialize_entry("switch_inline_query", query)
            }
            ButtonKind::SwitchInlineQueryCurrentChat(query) => {
                map.serialize_entry("switch_inline_query_current_chat", query)
            }
            ButtonKind::CallbackGame(game) => {
                map.serialize_entry("callback_game", &game)
            }
            ButtonKind::Pay(pay) => map.serialize_entry("pay", &pay),
        }?;

        map.end()
    }
}

impl<'a> Keyboard<'a> {
    /// Constructs an inline `Keyboard`.
    pub const fn new(buttons: Markup<'a>) -> Self {
        Self {
            inline_keyboard: buttons,
        }
    }
}

impl<'a> From<Markup<'a>> for Keyboard<'a> {
    fn from(markup: Markup<'a>) -> Self {
        Self::new(markup)
    }
}
