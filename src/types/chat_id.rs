/// Represents possible ways to specify the destination chat.
#[derive(Debug, PartialEq, Clone)]
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
