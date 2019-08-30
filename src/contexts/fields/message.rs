use super::Context;
use crate::types::{message, Chat, User};

/// A general trait for all message contexts.
pub trait Message<C>: Context<C> {
    /// ID of the message.
    fn message_id(&self) -> message::Id;
    /// The author of the message.
    fn from(&self) -> Option<&User>;
    /// The timestamp of the message.
    fn date(&self) -> i64;
    /// The chat to which the message was sent.
    fn chat(&self) -> &Chat;
}
