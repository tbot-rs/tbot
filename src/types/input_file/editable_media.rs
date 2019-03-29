use super::*;

/// Represents media that can be used to edit a message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

impl Serialize for EditableMedia<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            EditableMedia::Animation(animation) => animation.serialize(s),
            EditableMedia::Audio(audio) => audio.serialize(s),
            EditableMedia::Document(document) => document.serialize(s),
            EditableMedia::Photo(photo) => photo.serialize(s),
            EditableMedia::Video(video) => video.serialize(s),
        }
    }
}
