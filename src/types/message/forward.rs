//! Types representing a forward.

use super::Id;
use crate::types::{Chat, User};
use is_macro::Is;

/// Represents a forward source.
#[derive(Debug, PartialEq, Clone, Is)]
#[non_exhaustive]
pub enum From {
    /// The forward is from a user.
    User(User),
    /// The forward is from a user who decided to hide their profile.
    HiddenUser(String),
    /// The forward is from a channel.
    #[non_exhaustive]
    Channel {
        /// Information about the channel.
        chat: Box<Chat>,
        /// The ID of the original message.
        message_id: Id,
        /// The author's signature.
        signature: Option<String>,
    },
    /// The forward is from an anonymous group admin.
    #[non_exhaustive]
    AnonymousAdmin {
        /// Information about the group.
        chat: Box<Chat>,
        /// The admin's signature.
        signature: Option<String>,
    },
}

/// Represents forward information.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Forward {
    /// The author of the original message.
    pub from: From,
    /// The timestamp of the original message.
    pub date: i64,
}
