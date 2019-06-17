use serde::Deserialize;

/// Represents a [`ChatPhoto`].
///
/// [`ChatPhoto`]: https://core.telegram.org/bots/api#chatphoto
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Photo {
    /// The file ID of the small photo.
    pub small_file_id: String,
    /// THe file ID of the big photo.
    pub big_file_id: String,
}
