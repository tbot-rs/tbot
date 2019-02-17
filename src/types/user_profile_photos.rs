use super::*;

/// Represents [`UserProfilePhotos`].
///
/// [`UserProfilePhotos`]: https://core.telegram.org/bots/api#userprofilephotos
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct UserProfilePhotos {
    /// Overall amount of user's photos.
    pub total_count: u64,
    /// Vector of photos in different sizes.
    pub photos: Vec<Vec<PhotoSize>>,
}
