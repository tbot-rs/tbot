//! Types used as parameters, mainly for methods.

mod allowed_updates;
mod callback_action;
mod chat_id;
mod photo;
pub mod poll;
mod text;
pub mod tip;

pub(crate) use text::ParseMode;
pub use {
    allowed_updates::AllowedUpdates,
    callback_action::CallbackAction,
    chat_id::{ChatId, ImplicitChatId},
    photo::Photo,
    text::Text,
};
