use crate::types::{
    file::{self, id::AsFileId},
    value::FileId,
};
use serde::Deserialize;

/// Represents a [`PassportFile`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportfile
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct File {
    /// The ID of the file.
    pub id: file::Id,
    /// The size of the file.
    pub size: usize,
    /// The date of the file.
    pub date: i64,
}

impl crate::internal::Sealed for File {}
impl crate::internal::Sealed for &'_ File {}

impl<'a> AsFileId<'a> for File {
    fn as_file_id(self) -> FileId<'a> {
        self.id.into()
    }
}

impl<'a> AsFileId<'a> for &'a File {
    fn as_file_id(self) -> FileId<'a> {
        self.id.as_ref().into()
    }
}
