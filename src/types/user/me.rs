use super::User;
use serde::Deserialize;

/// Represents a Bot object that returned in [`GetMe`] method.
///
/// [`GetMe`]: ../../methods/struct.GetMe.html
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Me {
    /// The main information about the bot.
    #[serde(flatten)]
    pub user: User,
    /// `true` if the bot can join groups.
    pub can_join_groups: bool,
    /// `true` if the bot can read all messages in groups.
    pub can_read_all_group_messages: bool,
    /// `true` if the bot supports inline mode.
    pub supports_inline_queries: bool,
}
