use crate::types::{
    file::{self, id::AsFileId},
    PhotoSize,
};
use serde::Deserialize;

/// Represents an [`Animation`].
///
/// [`Animation`]: https://core.telegram.org/bots/api#animation
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Animation {
    /// The file ID of the animation.
    pub file_id: file::Id<'static>,
    /// The unique ID of the animation.
    pub file_unique_id: String,
    /// The width of the animation.
    pub width: u32,
    /// The height of the animation.
    pub height: u32,
    /// The duration of the animation.
    pub duration: u32,
    /// The thumb of the animation.
    pub thumb: Option<PhotoSize>,
    /// The MIME type of the animation.
    pub mime_type: Option<String>,
    /// The file size of the animation.
    pub file_size: Option<u32>,
}

impl crate::internal::Sealed for Animation {}

impl<'a> AsFileId<'a> for Animation {
    #[must_use]
    fn as_file_id(&self) -> file::id::Id<'_> {
        self.file_id.as_borrowed()
    }
}
