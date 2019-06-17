use super::*;

/// Represents a media that can be sent in a group (aka albums).
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
// todo: #[non_exhaustive]
pub enum GroupMedia<'a> {
    /// A group's photo.
    Photo(Photo<'a>),
    /// A group's video.
    Video(Video<'a>),
}

impl GroupMedia<'_> {
    /// Checks if `self` is `Photo`.
    pub fn is_photo(&self) -> bool {
        match self {
            GroupMedia::Photo(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Video`.
    pub fn is_video(&self) -> bool {
        match self {
            GroupMedia::Video(..) => true,
            _ => false,
        }
    }
}

impl<'a> From<Photo<'a>> for GroupMedia<'a> {
    fn from(photo: Photo<'a>) -> Self {
        GroupMedia::Photo(photo)
    }
}

impl<'a> From<Video<'a>> for GroupMedia<'a> {
    fn from(video: Video<'a>) -> Self {
        GroupMedia::Video(video)
    }
}

impl<'a> serde::Serialize for GroupMedia<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            GroupMedia::Photo(photo) => photo.serialize(s),
            GroupMedia::Video(video) => video.serialize(s),
        }
    }
}
