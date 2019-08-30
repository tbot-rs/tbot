use super::MediaMessage;
use crate::types::{self, PhotoSize};

/// A general trait for animation messages.
pub trait Animation<C>: MediaMessage<C> {
    /// The animation of the message.
    fn animation(&self) -> &types::Animation;
}

/// A general trait for audio messages.
pub trait Audio<C>: MediaMessage<C> {
    /// The audio of the message.
    fn audio(&self) -> &types::Audio;
}

/// A general trait for document messages.
pub trait Document<C>: MediaMessage<C> {
    /// The document of the message.
    fn document(&self) -> &types::Document;
}

/// A general trait for location messages.
pub trait Location<C>: MediaMessage<C> {
    /// The location of the message.
    fn location(&self) -> &types::Location;
}

/// A general trait for photo messages.
pub trait Photo<C>: MediaMessage<C> {
    /// The photo of the message.
    fn photo(&self) -> &[PhotoSize];
}

/// A general trait for video messages.
pub trait Video<C>: MediaMessage<C> {
    /// The video of the message.
    fn video(&self) -> &types::Video;
}
