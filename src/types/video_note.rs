use crate::types::{
    file::{self, id::AsFileId},
    PhotoSize,
};
use serde::Deserialize;

/// Represents a [`VideoNote`].
///
/// [`VideoNote`]: https://core.telegram.org/bots/api#videonote
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct VideoNote {
    /// The file ID of the video note.
    pub file_id: file::Id,
    /// The length of the video note.
    pub length: u32,
    /// The duration of the video note.
    pub duration: u32,
    /// The thumb ID of the video note.
    pub thumb: Option<PhotoSize>,
    /// The file size of the video note.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for VideoNote {}

impl AsFileId for VideoNote {
    fn as_file_id(&self) -> file::id::Ref<'_> {
        self.file_id.as_ref()
    }
}
