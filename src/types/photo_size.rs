use crate::types::file::{self, id::AsFileId};
use serde::Deserialize;

/// Represents a [`PhotoSize`].
///
/// [`PhotoSize`]: https://core.telegram.org/bots/api#photosize
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct PhotoSize {
    /// The file ID of the photo.
    pub file_id: file::Id,
    /// The unique ID of the photo.
    pub file_unique_id: String,
    /// The width of the photo.
    pub width: u32,
    /// The height of the photo.
    pub height: u32,
    /// The file size of the photo.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for PhotoSize {}

impl<'a> AsFileId<'a> for PhotoSize {
    #[must_use]
    fn as_file_id(&self) -> file::id::Ref<'_> {
        self.file_id.as_ref()
    }
}
