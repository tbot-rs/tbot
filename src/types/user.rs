//! Types related to users.

use serde::Deserialize;

mod id;
mod me;
mod profile_photos;

pub use {id::Id, me::Me, profile_photos::ProfilePhotos};

/// Represents a [`User`].
///
/// [`User`]: https://core.telegram.org/bots/api#user
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// The ID of the user.
    pub id: Id,
    /// `true` if the user is a bot.
    pub is_bot: bool,
    /// The first name of the user.
    pub first_name: String,
    /// The last name of the user.
    pub last_name: Option<String>,
    /// The username of the user.
    pub username: Option<String>,
    /// The language of the user.
    pub language_code: Option<String>,
}
