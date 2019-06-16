use serde::Serialize;

/// Represents possible ways to specify the destination chat.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
pub enum ChatId<'a> {
    /// The ID of a chat.
    Id(i64),
    /// The `@username` of a chat.
    Username(&'a str),
}

impl<'a> From<i64> for ChatId<'a> {
    fn from(id: i64) -> ChatId<'a> {
        ChatId::Id(id)
    }
}

impl<'a> From<&'a str> for ChatId<'a> {
    fn from(username: &'a str) -> ChatId<'a> {
        ChatId::Username(username)
    }
}
