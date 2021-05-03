use crate::types::{chat, user};
use is_macro::Is;
use serde::Serialize;

/// Represents possible ways to specify the destination chat.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Is)]
#[serde(untagged)]
#[non_exhaustive]
#[must_use]
pub enum ChatId {
    /// The ID of a chat.
    Id(chat::Id),
    /// The `@username` of a chat.
    Username(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        Self::Id(chat::Id(id))
    }
}

impl From<chat::Id> for ChatId {
    fn from(id: chat::Id) -> Self {
        Self::Id(id)
    }
}

impl From<user::Id> for ChatId {
    fn from(id: user::Id) -> Self {
        Self::Id(chat::Id(id.0))
    }
}

impl<'a> From<String> for ChatId {
    fn from(username: String) -> Self {
        Self::Username(username)
    }
}

impl<'a> From<&'a String> for ChatId {
    fn from(username: &'a String) -> Self {
        Self::Username(username.to_owned())
    }
}

impl<'a> From<&'a str> for ChatId {
    fn from(username: &'a str) -> Self {
        Self::Username(username.to_owned())
    }
}

impl<'a> From<&'a ChatId> for ChatId {
    fn from(chat_id: &'a Self) -> Self {
        chat_id.clone()
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
pub trait ImplicitChatId: Into<ChatId> {}

impl ImplicitChatId for ChatId {}
impl ImplicitChatId for &'_ ChatId {}
impl ImplicitChatId for chat::Id {}
impl ImplicitChatId for user::Id {}
