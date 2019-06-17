use super::{Animation, Audio, Document, Photo, Video};
use serde::Serialize;

/// Represents media that can be used to edit a message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
// todo: #[non_exhaustive]
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

impl EditableMedia<'_> {
    /// Checks if `self` is `Animation`.
    pub fn is_animation(&self) -> bool {
        match self {
            EditableMedia::Animation(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Audio`.
    pub fn is_audio(&self) -> bool {
        match self {
            EditableMedia::Audio(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Document`.
    pub fn is_document(&self) -> bool {
        match self {
            EditableMedia::Document(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Photo`.
    pub fn is_photo(&self) -> bool {
        match self {
            EditableMedia::Photo(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Video`.
    pub fn is_video(&self) -> bool {
        match self {
            EditableMedia::Video(..) => true,
            _ => false,
        }
    }
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
