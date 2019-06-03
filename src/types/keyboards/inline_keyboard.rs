use super::*;
use serde::ser::SerializeMap;
use InlineButtonType::{
    CallbackData, Pay, SwitchInlineQuery, SwitchInlineQueryCurrentChat, Url,
};

/// Represents different types an inline button can be.
///
/// Complete descriptions can be found in [Bots API docs][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InlineButtonType<'a> {
    /// Represents a URL button.
    Url(&'a str),
    /// Represents callback data.
    CallbackData(&'a str),
    /// Represents query inserted when switched to inline.
    SwitchInlineQuery(&'a str),
    /// Represents query inserted when switched to inline in the curent chat.
    SwitchInlineQueryCurrentChat(&'a str),
    /// Represent a description of the game to be laucnhed.
    CallbackGame(CallbackGame),
    /// If `true`, a pay button is sent.
    Pay(bool),
}

/// Represents an [`InlineKeyboardButton`].
///
/// [`InlineKeyboardButton`]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct InlineButton<'a> {
    text: &'a str,
    button_type: InlineButtonType<'a>,
}

impl<'a> InlineButton<'a> {
    /// Constructs a new `InlineButton`.
    pub const fn new(text: &'a str, button_type: InlineButtonType<'a>) -> Self {
        Self {
            text,
            button_type,
        }
    }
}

impl serde::Serialize for InlineButton<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(Some(2))?;

        map.serialize_entry("text", self.text)?;

        match self.button_type {
            Url(url) => map.serialize_entry("url", url),
            CallbackData(callback_data) => {
                map.serialize_entry("callback_data", callback_data)
            }
            SwitchInlineQuery(query) => {
                map.serialize_entry("switch_inline_query", query)
            }
            SwitchInlineQueryCurrentChat(query) => {
                map.serialize_entry("switch_inline_query_current_chat", query)
            }
            InlineButtonType::CallbackGame(game) => {
                map.serialize_entry("callback_game", &game)
            }
            Pay(pay) => map.serialize_entry("pay", &pay),
        }?;

        map.end()
    }
}

/// Represents an [`InlineKeyboardMarkup`].
///
/// [`InlineKeyboardMarkup`]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct InlineKeyboard<'a> {
    inline_keyboard: Vec<Vec<InlineButton<'a>>>,
}

impl<'a> InlineKeyboard<'a> {
    /// Constructs a new `InlineKeyboard`.
    pub const fn new(buttons: Vec<Vec<InlineButton<'a>>>) -> Self {
        Self {
            inline_keyboard: buttons,
        }
    }
}
