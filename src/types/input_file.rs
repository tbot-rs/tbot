//! Types representing uploadable media.

use crate::types::file;

mod animation;
mod audio;
mod chat_photo;
mod document;
mod editable_media;
mod group_media;
mod photo;
mod png_sticker;
mod sticker;
mod sticker_for_sticker_set;
mod sticker_set_thumb;
mod tgs_sticker;
mod thumb;
mod video;
mod video_note;
mod voice;

pub(crate) use group_media::Album;
pub use {
    animation::Animation, audio::Audio, chat_photo::ChatPhoto,
    document::Document, editable_media::EditableMedia, group_media::GroupMedia,
    photo::Photo, png_sticker::PngSticker, sticker::Sticker,
    sticker_for_sticker_set::StickerForStickerSet,
    sticker_set_thumb::StickerSetThumb, tgs_sticker::TgsSticker, thumb::Thumb,
    video::Video, video_note::VideoNote, voice::Voice,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum InputFile<'a> {
    File { filename: &'a str, bytes: &'a [u8] },
    Url(&'a str),
    Id(file::id::Ref<'a>),
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
            InputFile::File { .. } => {
                serializer.serialize_str(&format!("attach://{}", name))
            }
            InputFile::Url(file) | InputFile::Id(file::id::Ref(file)) => {
                serializer.serialize_str(file)
            }
        }
    }

    const fn with_name(self, name: &'a str) -> WithName<'a> {
        WithName { file: self, name }
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
