use crate::types::{Chat, User};
use is_macro::Is;

/// Represents the author of a message.
#[derive(Debug, PartialEq, Clone, Is)]
pub enum From {
    /// A user sent this message. Reflects the `from` field.
    User(User),
    /// A channel or a suoergroup sent this message.
    /// Reflects the `sender_chat` field.
    Chat(Chat),
}
