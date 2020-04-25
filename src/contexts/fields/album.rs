use super::MediaMessage;

/// A general trait for album items.
pub trait Album: MediaMessage {
    /// The ID of the album.
    fn media_group_id(&self) -> Option<&str>;
}
