use super::{Animation, Audio, Document, Photo, Video};
use serde::Serialize;

/// Represents media that can be used to edit a message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
pub enum EditableMedia<'a> {
    /// An animation that will replace the old media.
    Animation(&'a Animation<'a>),
    /// An audio that will replace the old media.
    Audio(&'a Audio<'a>),
    /// A document that will replace the old media.
    Document(&'a Document<'a>),
    /// A photo that will replace the old media.
    Photo(&'a Photo<'a>),
    /// A video that will replace the old media.
    Video(&'a Video<'a>),
}

impl<'a> From<&'a Animation<'a>> for EditableMedia<'a> {
    fn from(animation: &'a Animation<'a>) -> Self {
        EditableMedia::Animation(animation)
    }
}

impl<'a> From<&'a Audio<'a>> for EditableMedia<'a> {
    fn from(audio: &'a Audio<'a>) -> Self {
        EditableMedia::Audio(audio)
    }
}

impl<'a> From<&'a Document<'a>> for EditableMedia<'a> {
    fn from(document: &'a Document<'a>) -> Self {
        EditableMedia::Document(document)
    }
}

impl<'a> From<&'a Photo<'a>> for EditableMedia<'a> {
    fn from(photo: &'a Photo<'a>) -> Self {
        EditableMedia::Photo(photo)
    }
}

impl<'a> From<&'a Video<'a>> for EditableMedia<'a> {
    fn from(video: &'a Video<'a>) -> Self {
        EditableMedia::Video(video)
    }
}
