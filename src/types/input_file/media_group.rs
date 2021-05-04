use std::slice;

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
pub enum PhotoOrVideo {
    /// A group's photo.
    Photo(Photo),
    /// A group's video.
    Video(Video),
}

/// Represents a possible media group.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
#[must_use]
pub enum MediaGroup {
    /// An album of photos and videos.
    PhotosAndVideos(Vec<PhotoOrVideo>),
    /// A group of audios.
    Audios(Vec<Audio>),
    /// A group of documents.
    Documents(Vec<Document>),
}

pub enum AnyGroupMedia<'a> {
    Photo(&'a Photo),
    Video(&'a Video),
    Audio(&'a Audio),
    Document(&'a Document),
}

struct WithIndex<'a> {
    media: AnyGroupMedia<'a>,
    index: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Album(pub MediaGroup);

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

impl From<Photo> for PhotoOrVideo {
    fn from(photo: Photo) -> Self {
        Self::Photo(photo)
    }
}

impl From<Video> for PhotoOrVideo {
    fn from(video: Video) -> Self {
        Self::Video(video)
    }
}

impl MediaGroup {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::PhotosAndVideos(album) => album.len(),
            Self::Audios(audios) => audios.len(),
            Self::Documents(documents) => documents.len(),
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = AnyGroupMedia<'_>> {
        enum Iter<'a> {
            PhotosAndVideos(slice::Iter<'a, PhotoOrVideo>),
            Audios(slice::Iter<'a, Audio>),
            Documents(slice::Iter<'a, Document>),
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

impl From<Vec<PhotoOrVideo>> for MediaGroup {
    fn from(album: Vec<PhotoOrVideo>) -> Self {
        Self::PhotosAndVideos(album)
    }
}

impl From<Vec<Audio>> for MediaGroup {
    fn from(audios: Vec<Audio>) -> Self {
        Self::Audios(audios)
    }
}

impl From<Vec<Document>> for MediaGroup {
    fn from(documents: Vec<Document>) -> Self {
        Self::Documents(documents)
    }
}

impl Serialize for Album {
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
