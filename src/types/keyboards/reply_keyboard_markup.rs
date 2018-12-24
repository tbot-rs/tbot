use super::*;

/// Represents a [`KeyboardButton`].
///
/// [`KeyboardButton`]: https://core.telegram.org/bots/api#keyboardbutton
#[derive(Serialize, Debug, PartialEq, Clone)]
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
    pub fn new(text: &'a str) -> ReplyButton {
        ReplyButton {
            text,
            request_contact: None,
            request_localization: None,
        }
    }

    /// Sets `request_contact` to `Some(is_requested)`.
    pub fn request_contact(mut self, is_requested: bool) -> Self {
        self.request_contact = Some(is_requested);
        self
    }

    /// Sets `request_localization` to `Some(is_requested)`.
    pub fn request_localization(mut self, is_requested: bool) -> Self {
        self.request_localization = Some(is_requested);
        self
    }
}

/// Represents Telegram's [`ReplyKeyboardMarkup`].
///
/// [`ReplyKeyboardMarkup`]: https://core.telegram.org/bots/api#replykeyboardmarkup
#[derive(Serialize, Debug, PartialEq, Clone)]
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
    pub fn new(buttons: Vec<Vec<ReplyButton<'a>>>) -> ReplyKeyboard<'a> {
        ReplyKeyboard {
            keyboard: buttons,
            resize_keyboard: None,
            one_time_keyboard: None,
            selective: None,
        }
    }

    /// Sets `resize_keyboard` to `Some(is_resized)`.
    pub fn resize_keyboard(mut self, is_resized: bool) -> Self {
        self.resize_keyboard = Some(is_resized);
        self
    }

    /// Sets `one_time_keyboard` to `Some(is_one_time)`.
    pub fn one_time_keyboard(mut self, is_one_time: bool) -> Self {
        self.one_time_keyboard = Some(is_one_time);
        self
    }

    /// Sets `selective` to `Some(is_selective)`.
    pub fn selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}
