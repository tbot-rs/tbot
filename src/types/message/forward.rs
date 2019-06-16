//! Types representing a forward.

use crate::types::{Chat, User};

/// Represents a forward source.
#[derive(Debug, PartialEq, Clone)]
// todo: #[non_exhaustive]
pub enum From {
    /// The forward is from a user.
    User(User),
    /// The forward is from a user who decided to hide their profile.
    HiddenUser(String),
    /// The forward is from a channel.
    // todo: #[non_exhaustive]
    Channel {
        /// Information about the channel.
        chat: Chat,
        /// The ID of the original message.
        message_id: u32,
        /// The author's signature.
        signature: Option<String>,
    },
}

/// Represents forward information.
#[derive(Debug, PartialEq, Clone)]
// todo: #[non_exhaustive]
pub struct Forward {
    /// The author of the original message.
    pub from: From,
    /// The timestamp of the original message.
    pub date: i64,
}
