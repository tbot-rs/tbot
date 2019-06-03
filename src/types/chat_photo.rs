use super::*;

/// Represents a [`ChatPhoto`].
///
/// [`ChatPhoto`]: https://core.telegram.org/bots/api#chatphoto
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct ChatPhoto {
    /// The file ID of the small photo.
    pub small_file_id: String,
    /// THe file ID of the big photo.
    pub big_file_id: String,
}
