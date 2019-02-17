use super::*;

/// Represents possible chat types.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatTypes {
    /// The chat is private.
    Private,
    /// The chat is a group.
    Group,
    /// The chat is a supergroup.
    Supergroup,
    /// The chat is a channel.
    Channel,
}
