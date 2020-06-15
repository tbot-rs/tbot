use crate::types::{chat, user};
use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;
use std::ops::Deref;

/// Represents possible ways to specify the destination chat.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Is)]
#[serde(untagged)]
#[non_exhaustive]
#[must_use]
pub enum ChatId<'a> {
    /// The ID of a chat.
    Id(chat::Id),
    /// The `@username` of a chat.
    Username(Cow<'a, str>),
}

impl<'a> From<i64> for ChatId<'a> {
    fn from(id: i64) -> Self {
        ChatId::Id(chat::Id(id))
    }
}

impl<'a> From<chat::Id> for ChatId<'a> {
    fn from(id: chat::Id) -> Self {
        ChatId::Id(id)
    }
}

impl<'a> From<user::Id> for ChatId<'a> {
    fn from(id: user::Id) -> Self {
        ChatId::Id(chat::Id(id.0))
    }
}

impl<'a> From<&'a str> for ChatId<'a> {
    fn from(username: &'a str) -> Self {
        ChatId::Username(username.into())
    }
}

impl<'a> From<String> for ChatId<'a> {
    fn from(username: String) -> Self {
        ChatId::Username(username.into())
    }
}

impl<'a> From<&'a ChatId<'a>> for ChatId<'a> {
    fn from(chat_id: &'a Self) -> Self {
        match chat_id {
            ChatId::Id(id) => ChatId::Id(*id),
            ChatId::Username(username) => {
                ChatId::Username(username.deref().into())
            }
        }
    }
}

/// Allows certain types to be turned into `ChatId` implicitly.
///
/// Implicit turning is safe for chat ID wrappers. However, turning primitives
/// into `ChatId` implicitly is not safe, as the primitive might have a
/// different meaning. Because of that, we require to turn primitives into
/// `ChatId` explicitly.
#[allow(clippy::module_name_repetitions)]
// `parameters::chat_id::Implicit` is a less obvious name than
// `parameters::ImplicitChatId`
pub trait ImplicitChatId<'a>: Into<ChatId<'a>> {}
impl<'a> ImplicitChatId<'a> for ChatId<'a> {}
impl<'a> ImplicitChatId<'a> for &'a ChatId<'a> {}
impl ImplicitChatId<'_> for chat::Id {}
impl ImplicitChatId<'_> for user::Id {}
