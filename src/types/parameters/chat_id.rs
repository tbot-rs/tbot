use crate::types::{chat, user};
use serde::Serialize;

/// Represents possible ways to specify the destination chat.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
#[must_use]
pub enum ChatId<'a> {
    /// The ID of a chat.
    Id(chat::Id),
    /// The `@username` of a chat.
    Username(&'a str),
}

impl ChatId<'_> {
    /// Checks if `self` is `Id`.
    #[must_use]
    pub fn is_id(self) -> bool {
        match self {
            ChatId::Id(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Username`.
    #[must_use]
    pub fn is_username(self) -> bool {
        match self {
            ChatId::Username(..) => true,
            _ => false,
        }
    }
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
        ChatId::Username(username)
    }
}

/// Allows certian types to be turned into `ChatId` implicitly.
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
impl ImplicitChatId<'_> for chat::Id {}
impl ImplicitChatId<'_> for user::Id {}
