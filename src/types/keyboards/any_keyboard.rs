use super::*;
use serde::Serialize;

/// An enum of possible keyboards.
#[derive(Serialize)]
#[serde(untagged)]
pub enum AnyKeyboard<'a> {
    /// An inline keyboard.
    Inline(InlineKeyboard<'a>),
    /// A reply markup.
    ReplyMarkup(ReplyKeyboard<'a>),
    /// Removes reply markup.
    ReplyRemove(ReplyKeyboardRemove),
    /// Forces reply.
    ForceReply(ForceReply),
}

impl<'a> From<InlineKeyboard<'a>> for AnyKeyboard<'a> {
    fn from(keyboard: InlineKeyboard<'a>) -> AnyKeyboard<'a> {
        AnyKeyboard::Inline(keyboard)
    }
}

impl<'a> From<ReplyKeyboard<'a>> for AnyKeyboard<'a> {
    fn from(keyboard: ReplyKeyboard<'a>) -> AnyKeyboard<'a> {
        AnyKeyboard::ReplyMarkup(keyboard)
    }
}

impl<'a> From<ReplyKeyboardRemove> for AnyKeyboard<'a> {
    fn from(keyboard: ReplyKeyboardRemove) -> AnyKeyboard<'a> {
        AnyKeyboard::ReplyRemove(keyboard)
    }
}

impl<'a> From<ForceReply> for AnyKeyboard<'a> {
    fn from(keyboard: ForceReply) -> AnyKeyboard<'a> {
        AnyKeyboard::ForceReply(keyboard)
    }
}
