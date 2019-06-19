use crate::types::{chat, user};
use serde::Serialize;

/// Represents possible ways to specify the destination chat.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
// todo: #[non_exhaustive]
pub enum ChatId<'a> {
    /// The ID of a chat.
    Id(chat::Id),
    /// The `@username` of a chat.
    Username(&'a str),
}

impl ChatId<'_> {
    /// Checks if `self` is `Id`.
    pub fn is_id(self) -> bool {
        match self {
            ChatId::Id(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Username`.
    pub fn is_username(self) -> bool {
        match self {
            ChatId::Username(..) => true,
            _ => false,
        }
    }
}

impl<'a> From<i64> for ChatId<'a> {
    fn from(id: i64) -> ChatId<'a> {
        ChatId::Id(chat::Id(id))
    }
}

impl<'a> From<chat::Id> for ChatId<'a> {
    fn from(id: chat::Id) -> ChatId<'a> {
        ChatId::Id(id)
    }
}

impl<'a> From<user::Id> for ChatId<'a> {
    fn from(id: user::Id) -> ChatId<'a> {
        ChatId::Id(chat::Id(id.0))
    }
}

impl<'a> From<&'a str> for ChatId<'a> {
    fn from(username: &'a str) -> ChatId<'a> {
        ChatId::Username(username)
    }
}
