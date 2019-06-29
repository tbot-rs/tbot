//! Types representing uploadable media.

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum InputFile<'a> {
    File {
        filename: &'a str,
        bytes: &'a [u8],
    },
    Url(&'a str),
    Id(&'a str),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct WithName<'a> {
    pub(crate) file: InputFile<'a>,
    pub(crate) name: &'a str,
}

impl<'a> InputFile<'a> {
    fn serialize<S>(&self, serializer: S, name: &str) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            InputFile::File {
                ..
            } => serializer.serialize_str(&format!("attach://{}", name)),
            InputFile::Url(file) | InputFile::Id(file) => {
                serializer.serialize_str(file)
            }
        }
    }

    const fn with_name(self, name: &'a str) -> WithName<'a> {
        WithName {
            file: self,
            name,
        }
    }
}

impl<'a> serde::Serialize for WithName<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.file.serialize(serializer, self.name)
    }
}
