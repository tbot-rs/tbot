use crate::{
    contexts::fields,
    types::message::{inline_markup, Message},
};

/// A general trait for all non-service messages.
pub trait MediaMessage<C>: fields::Message<C> {
    /// The replied message.
    fn reply_to(&self) -> Option<&Message>;
    /// The author's signature, if enabled for the channel.
    fn author_signature(&self) -> Option<&str>;
    /// The inline keyboard attached to the message.
    fn reply_markup(&self) -> Option<&inline_markup::Keyboard>;
}
