use super::MediaMessage;
use crate::types::message;

/// A general trait for messages that _can_ be a forward.
pub trait Forward<C>: MediaMessage<C> {
    /// The origin of the message if it's a forward.
    fn forward(&self) -> Option<&message::Forward>;
}
