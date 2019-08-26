//! Types representing reply keyboards.

use super::*;
use serde::ser::SerializeMap;

/// A shorthand for reply markup.
pub type Markup<'a> = &'a [&'a [Button<'a>]];

/// Represents a [`KeyboardButton`].
///
/// [`KeyboardButton`]: https://core.telegram.org/bots/api#keyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Button<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_localization: Option<bool>,
}

/// Represents a [`ReplyKeyboardMarkup`].
///
/// [`ReplyKeyboardMarkup`]: https://core.telegram.org/bots/api#replykeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Keyboard<'a> {
    keyboard: Markup<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

/// Represents a [`ReplyKeyboardRemove`].
///
/// [`ReplyKeyboardRemove`]: https://core.telegram.org/bots/api#replykeyboardremove
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[must_use]
pub struct Remove {
    // remove_keyboard is added when serializing
    selective: Option<bool>,
}

impl<'a> Button<'a> {
    /// Constructs a reply `Button`.
    pub const fn new(text: &'a str) -> Self {
        Self {
            text,
            request_contact: None,
            request_localization: None,
        }
    }

    /// Configures `request_contact`.
    pub fn request_contact(mut self, is_requested: bool) -> Self {
        self.request_contact = Some(is_requested);
        self
    }

    /// Configures `request_localization`.
    pub fn request_localization(mut self, is_requested: bool) -> Self {
        self.request_localization = Some(is_requested);
        self
    }
}

impl<'a> Keyboard<'a> {
    /// Constructs a reply `Keyboard`.
    pub const fn new(keyboard: Markup<'a>) -> Self {
        Self {
            keyboard,
            resize_keyboard: None,
            one_time_keyboard: None,
            selective: None,
        }
    }

    /// Configures `resize_keyboard`.
    pub fn resize_keyboard(mut self, is_resized: bool) -> Self {
        self.resize_keyboard = Some(is_resized);
        self
    }

    /// Configures `one_time_keyboard`.
    pub fn one_time_keyboard(mut self, is_one_time: bool) -> Self {
        self.one_time_keyboard = Some(is_one_time);
        self
    }

    /// Configures `selective`.
    pub fn selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}

impl<'a> From<Markup<'a>> for Keyboard<'a> {
    fn from(markup: Markup<'a>) -> Self {
        Self::new(markup)
    }
}

impl Remove {
    /// Constructs a `reply::Remove`.
    pub const fn new() -> Self {
        Self { selective: None }
    }

    /// Configures `selective`.
    pub fn selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}

impl serde::Serialize for Remove {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let len = if self.selective.is_some() { 2 } else { 1 };

        let mut map = s.serialize_map(Some(len))?;

        map.serialize_entry("remove_keyboard", &true)?;

        if let Some(selective) = self.selective {
            map.serialize_entry("selective", &selective)?;
        }

        map.end()
    }
}
