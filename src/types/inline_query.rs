use super::*;

/// Represents an inline query ID.
#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(transparent)]
pub struct InlineQueryId(String);

/// Represents an [`InlineQuery`].
///
/// [`InlineQuery`]: https://core.telegram.org/bots/api#inlinequery
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct InlineQuery {
    /// The ID of the query.
    pub id: InlineQueryId,
    /// The user who sent the query.
    pub from: User,
    /// The location of the user, if enabled and allowed.
    pub location: Option<Location>,
    /// The query itself.
    pub query: String,
    /// The offset of the result to be returned.
    pub offset: String,
}
