use crate::types::{
    file::{self, id::AsFileId},
    PhotoSize,
};
use serde::Deserialize;

/// Represents a [`Video`].
///
/// [`Video`]: https://core.telegram.org/bots/api#video
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Video {
    /// The file ID of the video.
    pub file_id: file::Id<'static>,
    /// The unique ID of the video.
    pub file_unique_id: String,
    /// The width of the video.
    pub width: u32,
    /// The height of the video.
    pub height: u32,
    /// The duration of the video.
    pub duration: u32,
    /// The thumb of the video.
    pub thumb: Option<PhotoSize>,
    /// The MIME type of the video.
    pub mime_type: Option<String>,
    /// The file size of the video.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for Video {}

impl<'a> AsFileId<'a> for Video {
    #[must_use]
    fn as_file_id(&self) -> file::id::Id<'_> {
        self.file_id.as_borrowed()
    }
}
