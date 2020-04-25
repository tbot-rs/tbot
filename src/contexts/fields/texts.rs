use super::MediaMessage;
use crate::types::message;

/// A general trait for text messages.
pub trait Text: MediaMessage {
    /// The text of the message.
    fn text(&self) -> &message::Text;
}

/// A general trait for messages with a caption.
pub trait Caption: MediaMessage {
    /// The caption of the message.
    fn caption(&self) -> &message::Text;
}

/// Unites [`Text`] and [`Caption`].
///
/// [`Text`]: ./trait.Text.html
/// [`Caption`]: ./trait.Caption.html
pub trait AnyText: MediaMessage {
    /// The text or the caption of the message.
    fn text(&self) -> &message::Text;
}
