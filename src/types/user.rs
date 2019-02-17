use super::*;

/// Represents a [`User`].
///
/// [`User`]: https://core.telegram.org/bots/api#user
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct User {
    /// The user's ID.
    pub id: i64,
    /// Whether the user is a bot.
    pub is_bot: bool,
    /// The user's first name.
    pub first_name: String,
    /// The user's last name.
    pub last_name: Option<String>,
    /// The user's username.
    pub username: Option<String>,
    /// The user's language.
    pub language_code: Option<String>,
}
