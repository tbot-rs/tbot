use super::*;
use crate::types::{file::id::AsFileId, value::FileId};

/// Represents a [`PhotoSize`].
///
/// [`PhotoSize`]: https://core.telegram.org/bots/api#photosize
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct PhotoSize {
    /// The file ID of the photo.
    pub file_id: file::Id,
    /// The width of the photo.
    pub width: u32,
    /// The height of the photo.
    pub height: u32,
    /// The file size of the photo.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for PhotoSize {}
impl crate::internal::Sealed for &'_ PhotoSize {}

impl<'a> AsFileId<'a> for PhotoSize {
    fn as_file_id(self) -> FileId<'a> {
        self.file_id.into()
    }
}

impl<'a> AsFileId<'a> for &'a PhotoSize {
    fn as_file_id(self) -> FileId<'a> {
        self.file_id.as_ref().into()
    }
}
