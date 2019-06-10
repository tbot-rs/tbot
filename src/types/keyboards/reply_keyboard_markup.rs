use super::*;

/// Represents a [`KeyboardButton`].
///
/// [`KeyboardButton`]: https://core.telegram.org/bots/api#keyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct ReplyButton<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_localization: Option<bool>,
}

impl<'a> ReplyButton<'a> {
    /// Constructs a new `ReplyButton`.
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

/// Represents a [`ReplyKeyboardMarkup`].
///
/// [`ReplyKeyboardMarkup`]: https://core.telegram.org/bots/api#replykeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct ReplyKeyboard<'a> {
    keyboard: Vec<Vec<ReplyButton<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

impl<'a> ReplyKeyboard<'a> {
    /// Constructs a new `ReplyKeyboard`.
    pub const fn new(buttons: Vec<Vec<ReplyButton<'a>>>) -> Self {
        Self {
            keyboard: buttons,
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
