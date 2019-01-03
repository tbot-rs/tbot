use super::*;

mod animation;
mod document;
mod photo;
mod thumb;
mod video;
mod video_note;

pub use self::animation::*;
pub use self::document::*;
pub use self::photo::*;
pub use self::thumb::*;
pub use self::video::*;
pub use self::video_note::*;

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
