use super::*;
use crate::types::{file::id::AsFileId, value::FileId};

/// Represents a [`Document`].
///
/// [`Document`]: https://core.telegram.org/bots/api#document
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Document {
    /// The file ID of the document.
    pub file_id: file::Id,
    /// The thumb of the document.
    pub thumb: Option<PhotoSize>,
    /// The file name of the document.
    pub file_name: Option<String>,
    /// The MIME type of the document.
    pub mime_type: Option<String>,
    /// The file size of the document.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for Document {}
impl crate::internal::Sealed for &'_ Document {}

impl<'a> AsFileId<'a> for Document {
    fn as_file_id(self) -> FileId<'a> {
        self.file_id.into()
    }
}

impl<'a> AsFileId<'a> for &'a Document {
    fn as_file_id(self) -> FileId<'a> {
        self.file_id.as_ref().into()
    }
}
