use crate::types::PhotoSize;
use serde::Deserialize;

/// Represents [`UserProfilePhotos`].
///
/// [`UserProfilePhotos`]: https://core.telegram.org/bots/api#userprofilephotos
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
#[must_use]
pub struct ProfilePhotos {
    /// Overall amount of photos of the user.
    pub total_count: u32,
    /// Vector of photos in different sizes.
    pub photos: Vec<Vec<PhotoSize>>,
}
