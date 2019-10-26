//! Types representing inline keyboard markup coming from Telegram.
//!
//! The reason that we can't re-use the types from `types::keyboard` is that
//! those types are meant for serialization, and so they're compoud of
//! references. These references, however, would be a pain if deserialized.
//! That's why we mirrored the inline keyboard types coming from Telegram
//! without references.

use crate::types::callback::Game;
use serde::de::{
    self, Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor,
};
use std::fmt::{self, Formatter};

/// A shorthand for inline markup.
pub type Markup = Vec<Vec<Button>>;

/// Represents different types an inline button can be.
///
/// Complete descriptions can be found in [Bots API docs][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
// todo: #[non_exhaustive]
pub enum ButtonKind {
    /// Represents a URL button.
    Url(String),
    /// Represents callback data.
    CallbackData(String),
    /// Represents query inserted when switched to inline.
    SwitchInlineQuery(String),
    /// Represents query inserted when switched to inline in the curent chat.
    SwitchInlineQueryCurrentChat(String),
    /// Represent a description of the game to be laucnhed.
    CallbackGame(Game),
    /// if `true`, a pay button.
    Pay(bool),
}

impl ButtonKind {
    /// Checks if `self` is `Url`.
    pub fn is_url(&self) -> bool {
        match self {
            Self::Url(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `CallbackData`.
    pub fn is_callback_data(&self) -> bool {
        match self {
            Self::CallbackData(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SwitchInlineQuery`.
    pub fn is_switch_inline_query(&self) -> bool {
        match self {
            Self::SwitchInlineQuery(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `SwitchInlineQueryCurrentChat`.
    pub fn is_switch_inline_query_current_chat(&self) -> bool {
        // what a name

        match self {
            Self::SwitchInlineQueryCurrentChat(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `CallbackGame`.
    pub fn is_callback_game(&self) -> bool {
        match self {
            Self::CallbackGame(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Pay`.
    pub fn is_pay(&self) -> bool {
        match self {
            Self::Pay(..) => true,
            _ => false,
        }
    }
}

/// Represents an [`InlineKeyboardButton`].
///
/// [`InlineKeyboardButton`]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
// todo: #[non_exhaustive]
pub struct Button {
    /// The text of the button.
    pub text: String,
    /// The kind of the button.
    pub kind: ButtonKind,
}

/// Represents an [`InlineKeyboardMarkup`].
///
/// [`InlineKeyboardMarkup`]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Keyboard(pub Markup);

const INLINE_KEYBOARD: &str = "inline_keyboard";

struct KeyboardVisitor;

impl<'v> Visitor<'v> for KeyboardVisitor {
    type Value = Keyboard;

    fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "struct Keyboard")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'v>,
    {
        if let Some((INLINE_KEYBOARD, markup)) = map.next_entry()? {
            Ok(Keyboard(markup))
        } else {
            Err(de::Error::missing_field(INLINE_KEYBOARD))
        }
    }
}

impl<'de> Deserialize<'de> for Keyboard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Keyboard",
            &[INLINE_KEYBOARD],
            KeyboardVisitor,
        )
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
            ButtonKind::Url(url)
        } else if let Some(callback_data) = callback_data {
            ButtonKind::CallbackData(callback_data)
        } else if let Some(callback_game) = callback_game {
            ButtonKind::CallbackGame(callback_game)
        } else if let Some(switch_inline_query) = switch_inline_query {
            ButtonKind::SwitchInlineQuery(switch_inline_query)
        } else if let Some(switch_inline_query_current_chat) =
            switch_inline_query_current_chat
        {
            ButtonKind::SwitchInlineQueryCurrentChat(
                switch_inline_query_current_chat,
            )
        } else if let Some(pay) = pay {
            ButtonKind::Pay(pay)
        } else {
            return Err(serde::de::Error::custom(
                "Could not construct Button's kind",
            ));
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
