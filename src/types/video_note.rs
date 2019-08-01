use super::*;
use crate::types::{file::id::AsFileId, value::FileId};

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
impl crate::internal::Sealed for &'_ VideoNote {}

impl<'a> AsFileId<'a> for VideoNote {
    fn as_file_id(self) -> FileId<'a> {
        self.file_id.into()
    }
}

impl<'a> AsFileId<'a> for &'a VideoNote {
    fn as_file_id(self) -> FileId<'a> {
        self.file_id.as_ref().into()
    }
}
