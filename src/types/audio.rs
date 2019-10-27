use crate::types::{
    file::{self, id::AsFileId},
    PhotoSize,
};
use serde::Deserialize;

/// Represents an [`Audio`].
///
/// [`Audio`]: https://core.telegram.org/bots/api#audio
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Audio {
    /// The file ID of the audio.
    pub file_id: file::Id,
    /// The duration of the audio.
    pub duration: u32,
    /// The performer of the audio.
    pub performer: Option<String>,
    /// The title of the audio.
    pub title: Option<String>,
    /// The MIME type of the audio.
    pub mime_type: Option<String>,
    /// The file size of the audio.
    pub file_size: Option<u32>,
    /// The thumb of the audio.
    pub thumb: Option<PhotoSize>,
}

impl crate::internal::Sealed for Audio {}

impl AsFileId for Audio {
    fn as_file_id(&self) -> file::id::Ref<'_> {
        self.file_id.as_ref()
    }
}
