use super::*;
use crate::types::value::Ref;
use serde::{
    ser::{SerializeSeq, Serializer},
    Serialize,
};

/// Represents a media that can be sent in a group (aka albums).
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
// todo: #[non_exhaustive]
pub enum GroupMedia<'a> {
    /// A group's photo.
    Photo(Ref<'a, Photo<'a>>),
    /// A group's video.
    Video(Ref<'a, Video<'a>>),
}

struct WithIndex<'a> {
    media: &'a GroupMedia<'a>,
    index: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Album<'a>(pub &'a [Ref<'a, GroupMedia<'a>>]);

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

                Photo::serialize(photo.as_ref(), serializer, &name)
            }
            GroupMedia::Video(video) => {
                let video_name = format!("video_{}", index);
                let thumb_name = format!("thumb_{}", index);

                Video::serialize(
                    video.as_ref(),
                    serializer,
                    &video_name,
                    &thumb_name,
                )
            }
        }
    }
}

impl<'a> From<Photo<'a>> for GroupMedia<'a> {
    fn from(photo: Photo<'a>) -> Self {
        GroupMedia::Photo(photo.into())
    }
}

impl<'a> From<&'a Photo<'a>> for GroupMedia<'a> {
    fn from(photo: &'a Photo<'a>) -> Self {
        GroupMedia::Photo(photo.into())
    }
}

impl<'a> From<Video<'a>> for GroupMedia<'a> {
    fn from(video: Video<'a>) -> Self {
        GroupMedia::Video(video.into())
    }
}

impl<'a> From<&'a Video<'a>> for GroupMedia<'a> {
    fn from(video: &'a Video<'a>) -> Self {
        GroupMedia::Video(video.into())
    }
}

impl Serialize for Album<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

        for (index, media) in self.0.iter().enumerate() {
            let with_index = WithIndex {
                media: media.as_ref(),
                index,
            };

            seq.serialize_element(&with_index)?;
        }

        seq.end()
    }
}

impl Serialize for WithIndex<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.media.serialize(serializer, self.index)
    }
}
