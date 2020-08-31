use super::{Animation, Audio, Document, Photo, Video};
use is_macro::Is;
use serde::Serialize;

/// Represents media that can be used to edit a message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
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
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            EditableMedia::Animation(..) => "animation",
            EditableMedia::Audio(..) => "audio",
            EditableMedia::Document(..) => "document",
            EditableMedia::Photo(..) => "photo",
            EditableMedia::Video(..) => "video",
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
