use super::{Animation, Audio, Document, Photo, Video};
use is_macro::Is;
use serde::Serialize;

/// Represents media that can be used to edit a message.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Is)]
#[serde(untagged)]
#[non_exhaustive]
#[must_use]
pub enum EditableMedia {
    /// An animation that will replace the old media.
    Animation(Animation),
    /// An audio that will replace the old media.
    Audio(Audio),
    /// A document that will replace the old media.
    Document(Document),
    /// A photo that will replace the old media.
    Photo(Photo),
    /// A video that will replace the old media.
    Video(Video),
}

impl EditableMedia {
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Animation(..) => "animation",
            Self::Audio(..) => "audio",
            Self::Document(..) => "document",
            Self::Photo(..) => "photo",
            Self::Video(..) => "video",
        }
    }
}

impl From<Animation> for EditableMedia {
    fn from(animation: Animation) -> Self {
        Self::Animation(animation)
    }
}

impl From<Audio> for EditableMedia {
    fn from(audio: Audio) -> Self {
        Self::Audio(audio)
    }
}

impl From<Document> for EditableMedia {
    fn from(document: Document) -> Self {
        Self::Document(document)
    }
}

impl From<Photo> for EditableMedia {
    fn from(photo: Photo) -> Self {
        Self::Photo(photo)
    }
}

impl From<Video> for EditableMedia {
    fn from(video: Video) -> Self {
        Self::Video(video)
    }
}
