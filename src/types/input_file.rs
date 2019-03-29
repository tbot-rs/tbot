use super::*;

mod animation;
mod audio;
mod document;
mod editable_media;
mod group_media;
mod photo;
mod thumb;
mod video;
mod video_note;
mod voice;

pub use {
    animation::*, audio::*, document::*, editable_media::*, group_media::*,
    photo::*, thumb::*, video::*, video_note::*, voice::*,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum InputFile<'a> {
    File {
        name: String,
        filename: &'a str,
        bytes: &'a [u8],
    },
    Url(&'a str),
    Id(&'a str),
}

impl<'a> serde::Serialize for InputFile<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            InputFile::File {
                name,
                ..
            } => serializer.serialize_str(&format!("attach://{}", name)),
            InputFile::Url(file) | InputFile::Id(file) => {
                serializer.serialize_str(file)
            }
        }
    }
}
