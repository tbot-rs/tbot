use super::{Animation, Audio, Document, Photo, Video};
use serde::Serialize;

/// Represents media that can be used to edit a message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
#[must_use]
pub enum EditableMedia<'a> {
    /// An animation that will replace the old media.
    Animation(Animation<'a>),
    /// An audio that will replace the old media.
    Audio(Audio<'a>),
    /// A document that will replace the old media.
    Document(Document<'a>),
    /// A photo that will replace the old media.
    Photo(Photo<'a>),
    /// A video that will replace the old media.
    Video(Video<'a>),
}

impl EditableMedia<'_> {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            EditableMedia::Animation(..) => "animation",
            EditableMedia::Audio(..) => "audio",
            EditableMedia::Document(..) => "document",
            EditableMedia::Photo(..) => "photo",
            EditableMedia::Video(..) => "video",
        }
    }

    /// Checks if `self` is `Animation`.
    #[must_use]
    pub fn is_animation(&self) -> bool {
        match self {
            EditableMedia::Animation(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Audio`.
    #[must_use]
    pub fn is_audio(&self) -> bool {
        match self {
            EditableMedia::Audio(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Document`.
    #[must_use]
    pub fn is_document(&self) -> bool {
        match self {
            EditableMedia::Document(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Photo`.
    #[must_use]
    pub fn is_photo(&self) -> bool {
        match self {
            EditableMedia::Photo(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Video`.
    #[must_use]
    pub fn is_video(&self) -> bool {
        match self {
            EditableMedia::Video(..) => true,
            _ => false,
        }
    }
}

impl<'a> From<Animation<'a>> for EditableMedia<'a> {
    fn from(animation: Animation<'a>) -> Self {
        EditableMedia::Animation(animation)
    }
}

impl<'a> From<Audio<'a>> for EditableMedia<'a> {
    fn from(audio: Audio<'a>) -> Self {
        EditableMedia::Audio(audio)
    }
}

impl<'a> From<Document<'a>> for EditableMedia<'a> {
    fn from(document: Document<'a>) -> Self {
        EditableMedia::Document(document)
    }
}

impl<'a> From<Photo<'a>> for EditableMedia<'a> {
    fn from(photo: Photo<'a>) -> Self {
        EditableMedia::Photo(photo)
    }
}

impl<'a> From<Video<'a>> for EditableMedia<'a> {
    fn from(video: Video<'a>) -> Self {
        EditableMedia::Video(video)
    }
}
