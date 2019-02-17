/// Represents possible ways to specify the destination chat.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ChatId<'a> {
    /// A user's ID.
    Id(i64),
    /// A user's `@username`.
    Username(&'a str),
}

impl<'a> serde::Serialize for ChatId<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            ChatId::Id(id) => serializer.serialize_i64(*id),
            ChatId::Username(username) => serializer.serialize_str(username),
        }
    }
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
