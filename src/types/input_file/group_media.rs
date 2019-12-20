use super::{Photo, Video};
use serde::{
    ser::{SerializeSeq, Serializer},
    Serialize,
};

/// Represents a media that can be sent in a group (aka albums).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
#[must_use]
pub enum GroupMedia<'a> {
    /// A group's photo.
    Photo(Photo<'a>),
    /// A group's video.
    Video(Video<'a>),
}

struct WithIndex<'a> {
    media: GroupMedia<'a>,
    index: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Album<'a>(pub &'a [GroupMedia<'a>]);

impl GroupMedia<'_> {
    /// Checks if `self` is `Photo`.
    #[must_use]
    pub fn is_photo(&self) -> bool {
        match self {
            GroupMedia::Photo(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Video`.
    #[must_use]
    pub fn is_video(&self) -> bool {
        match self {
            GroupMedia::Video(..) => true,
            _ => false,
        }
    }

    fn serialize<S>(
        &self,
        serializer: S,
        index: usize,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GroupMedia::Photo(photo) => {
                let name = format!("photo_{}", index);

                photo.serialize(serializer, &name)
            }
            GroupMedia::Video(video) => {
                let video_name = format!("video_{}", index);
                let thumb_name = format!("thumb_{}", index);

                video.serialize(serializer, &video_name, &thumb_name)
            }
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

impl<'a> Serialize for Album<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

        for (index, media) in self.0.iter().enumerate() {
            let with_index = WithIndex {
                media: *media,
                index,
            };

            seq.serialize_element(&with_index)?;
        }

        seq.end()
    }
}

impl<'a> Serialize for WithIndex<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.media.serialize(serializer, self.index)
    }
}
