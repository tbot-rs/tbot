//! Types representing uploadable media.

use crate::types::value::{self, Bytes, FileId, Ref};
use serde::{Serialize, Serializer};

mod animation;
mod audio;
mod chat_photo;
mod document;
mod editable_media;
mod group_media;
mod photo;
mod png_sticker;
mod sticker;
mod thumb;
mod video;
mod video_note;
mod voice;

pub use {
    animation::*, audio::*, chat_photo::*, document::*, editable_media::*,
    group_media::*, photo::*, png_sticker::*, sticker::*, thumb::*, video::*,
    video_note::*, voice::*,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum InputFile<'a> {
    File {
        filename: value::String<'a>,
        bytes: Bytes<'a>,
    },
    Url(value::String<'a>),
    Id(FileId<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct WithName<'a> {
    pub(crate) file: Ref<'a, InputFile<'a>>,
    pub(crate) name: &'a str,
}

impl<'a> InputFile<'a> {
    fn serialize<S>(&self, serializer: S, name: &str) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InputFile::File {
                ..
            } => serializer.serialize_str(&format!("attach://{}", name)),
            InputFile::Url(url) => url.serialize(serializer),
            InputFile::Id(id) => id.serialize(serializer),
        }
    }

    fn borrow_with_name(&'a self, name: &'a str) -> WithName<'a> {
        WithName {
            file: self.into(),
            name,
        }
    }

    fn own_with_name(self, name: &'a str) -> WithName<'a> {
        WithName {
            file: self.into(),
            name,
        }
    }
}

impl<'a> Serialize for WithName<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.file.as_ref().serialize(serializer, self.name)
    }
}
