use super::Context;
use crate::types::{
    self,
    message::{self, inline_markup},
    Chat, User,
};

/// A general trait for all message contexts.
pub trait Message: Context {
    /// ID of the message.
    fn message_id(&self) -> message::Id;
    /// The author of the message.
    fn from(&self) -> Option<&message::From>;
    /// The timestamp of the message.
    fn date(&self) -> i64;
    /// The chat to which the message was sent.
    fn chat(&self) -> &Chat;
}

/// A general trait for all non-service messages.
pub trait MediaMessage: Message {
    /// The replied message.
    fn reply_to(&self) -> Option<&types::Message>;
    /// The author's signature, if enabled for the channel.
    fn author_signature(&self) -> Option<&str>;
    /// The inline keyboard attached to the message.
    fn reply_markup(&self) -> Option<&inline_markup::Keyboard>;
    /// The bot via which the message was sent.
    fn via_bot(&self) -> Option<&User>;
}

/// A general trait for messages that _can_ be a forward.
pub trait Forward: MediaMessage {
    /// The origin of the message if it's a forward.
    fn forward(&self) -> Option<&message::Forward>;
}

/// A general trait for edited messages.
pub trait EditedMessage: MediaMessage {
    /// The last time when the message was edited.
    fn edit_date(&self) -> i64;
}
