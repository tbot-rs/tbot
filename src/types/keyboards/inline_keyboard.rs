use super::*;

/// Represents different types an inline button can be. Complete descriptions
#[derive(Debug, PartialEq, Clone)]
pub enum InlineButtonType<'a> {
    /// Represents a URL button.
    Url(&'a str),
    /// Represents some callback data.
    CallbackData(&'a str),
    /// Represets query inserted when switched to inline.
    SwitchInlineQuery(&'a str),
    /// Represets query inserted when switched to inline in the curent chat.
    SwitchInlineQueryCurrentChat(&'a str),
    /// Will represent a description of the game to be laucnhed one day.
    CallbackGame(CallbackGame),
    /// If `true`, a pay button is sent.
    Pay(bool),
}

/// Represents an [`InlineKeyboardButton`].
///
/// [`InlineKeybaordButton`]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct InlineButton<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_data: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query_current_chat: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_game: Option<CallbackGame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay: Option<bool>,
}

impl<'a> InlineButton<'a> {
    /// Constructs a new `InlineButton`.
    #[must_use]
    pub fn new(
        text: &'a str,
        button_type: InlineButtonType<'a>,
    ) -> InlineButton<'a> {
        macro_rules! get {
            ($type:ident) => {
                if let InlineButtonType::$type(value) = button_type {
                    Some(value)
                } else {
                    None
                }
            };
        }

        InlineButton {
            text,
            url: get!(Url),
            callback_data: get!(CallbackData),
            switch_inline_query: get!(SwitchInlineQuery),
            switch_inline_query_current_chat: get!(
                SwitchInlineQueryCurrentChat
            ),
            callback_game: get!(CallbackGame),
            pay: get!(Pay),
        }
    }
}

/// Represents an [`InlineKeyboardMarkup`].
///
/// [`InlineKeyboardMarkup`]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct InlineKeyboard<'a> {
    inline_keyboard: Vec<Vec<InlineButton<'a>>>,
}

impl<'a> InlineKeyboard<'a> {
    /// Constructs a new `InlineKeyboard`.
    pub fn new(buttons: Vec<Vec<InlineButton<'a>>>) -> InlineKeyboard<'a> {
        InlineKeyboard {
            inline_keyboard: buttons,
        }
    }
}
