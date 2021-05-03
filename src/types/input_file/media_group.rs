use std::{borrow::Cow, slice};

use super::{Audio, Document, Photo, Video};
use is_macro::Is;
use serde::{
    ser::{SerializeSeq, Serializer},
    Serialize,
};

/// Represents a media that can be sent in a group (aka albums).
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
#[must_use]
pub enum PhotoOrVideo<'a> {
    /// A group's photo.
    Photo(Photo<'a>),
    /// A group's video.
    Video(Video<'a>),
}

/// Represents a possible media group.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
#[must_use]
pub enum MediaGroup<'a> {
    /// An album of photos and videos.
    PhotosAndVideos(Cow<'a, [PhotoOrVideo<'a>]>),
    /// A group of audios.
    Audios(Cow<'a, [Audio<'a>]>),
    /// A group of documents.
    Documents(Cow<'a, [Document<'a>]>),
}

pub enum AnyGroupMedia<'a> {
    Photo(&'a Photo<'a>),
    Video(&'a Video<'a>),
    Audio(&'a Audio<'a>),
    Document(&'a Document<'a>),
}

struct WithIndex<'a> {
    media: AnyGroupMedia<'a>,
    index: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Album<'a>(pub MediaGroup<'a>);

impl AnyGroupMedia<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
        index: usize,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Photo(photo) => {
                let name = format!("photo_{}", index);
                photo.serialize_with_name(serializer, &name)
            }
            Self::Video(video) => {
                let video_name = format!("video_{}", index);
                let thumb_name = format!("thumb_{}", index);
                video.serialize_with_names(serializer, &video_name, &thumb_name)
            }
            Self::Audio(audio) => {
                let audio_name = format!("audio_{}", index);
                let thumb_name = format!("thumb_{}", index);
                audio.serialize_with_names(serializer, &audio_name, &thumb_name)
            }
            Self::Document(document) => {
                let document_name = format!("document_{}", index);
                let thumb_name = format!("thumb_{}", index);
                document.serialize_with_names(
                    serializer,
                    &document_name,
                    &thumb_name,
                )
            }
        }
    }
}

impl<'a> From<Photo<'a>> for PhotoOrVideo<'a> {
    fn from(photo: Photo<'a>) -> Self {
        Self::Photo(photo)
    }
}

impl<'a> From<Video<'a>> for PhotoOrVideo<'a> {
    fn from(video: Video<'a>) -> Self {
        Self::Video(video)
    }
}

impl MediaGroup<'_> {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::PhotosAndVideos(album) => album.len(),
            Self::Audios(audios) => audios.len(),
            Self::Documents(documents) => documents.len(),
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = AnyGroupMedia<'_>> {
        enum Iter<'a> {
            PhotosAndVideos(slice::Iter<'a, PhotoOrVideo<'a>>),
            Audios(slice::Iter<'a, Audio<'a>>),
            Documents(slice::Iter<'a, Document<'a>>),
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = AnyGroupMedia<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::PhotosAndVideos(iter) => {
                        iter.next().map(|media| match media {
                            PhotoOrVideo::Photo(photo) => {
                                AnyGroupMedia::Photo(photo)
                            }
                            PhotoOrVideo::Video(video) => {
                                AnyGroupMedia::Video(video)
                            }
                        })
                    }
                    Self::Audios(iter) => iter.next().map(AnyGroupMedia::Audio),
                    Self::Documents(iter) => {
                        iter.next().map(AnyGroupMedia::Document)
                    }
                }
            }
        }

        match self {
            Self::PhotosAndVideos(album) => Iter::PhotosAndVideos(album.iter()),
            Self::Audios(audios) => Iter::Audios(audios.iter()),
            Self::Documents(documents) => Iter::Documents(documents.iter()),
        }
    }
}

impl<'a> From<&'a [PhotoOrVideo<'a>]> for MediaGroup<'a> {
    fn from(album: &'a [PhotoOrVideo<'a>]) -> Self {
        MediaGroup::PhotosAndVideos(Cow::Borrowed(album))
    }
}

impl<'a> From<&'a Vec<PhotoOrVideo<'a>>> for MediaGroup<'a> {
    fn from(album: &'a Vec<PhotoOrVideo<'a>>) -> Self {
        MediaGroup::PhotosAndVideos(Cow::Borrowed(album.as_slice()))
    }
}

impl<'a> From<Vec<PhotoOrVideo<'a>>> for MediaGroup<'a> {
    fn from(album: Vec<PhotoOrVideo<'a>>) -> Self {
        MediaGroup::PhotosAndVideos(Cow::Owned(album))
    }
}

impl<'a> From<&'a [Audio<'a>]> for MediaGroup<'a> {
    fn from(audios: &'a [Audio<'a>]) -> Self {
        MediaGroup::Audios(Cow::Borrowed(audios))
    }
}

impl<'a> From<&'a Vec<Audio<'a>>> for MediaGroup<'a> {
    fn from(audios: &'a Vec<Audio<'a>>) -> Self {
        MediaGroup::Audios(Cow::Borrowed(audios.as_slice()))
    }
}

impl<'a> From<Vec<Audio<'a>>> for MediaGroup<'a> {
    fn from(audios: Vec<Audio<'a>>) -> Self {
        MediaGroup::Audios(Cow::Owned(audios))
    }
}

impl<'a> From<&'a [Document<'a>]> for MediaGroup<'a> {
    fn from(documents: &'a [Document<'a>]) -> Self {
        MediaGroup::Documents(Cow::Borrowed(documents))
    }
}

impl<'a> From<&'a Vec<Document<'a>>> for MediaGroup<'a> {
    fn from(documents: &'a Vec<Document<'a>>) -> Self {
        MediaGroup::Documents(Cow::Borrowed(documents.as_slice()))
    }
}

impl<'a> From<Vec<Document<'a>>> for MediaGroup<'a> {
    fn from(documents: Vec<Document<'a>>) -> Self {
        MediaGroup::Documents(Cow::Owned(documents))
    }
}

impl<'a> Serialize for Album<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

        for (index, media) in self.0.iter().enumerate() {
            let with_index = WithIndex { media, index };

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
