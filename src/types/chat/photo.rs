use crate::types::file;
use serde::Deserialize;

/// Represents a [`ChatPhoto`].
///
/// [`ChatPhoto`]: https://core.telegram.org/bots/api#chatphoto
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Photo {
    /// The file ID of the small photo.
    #[serde(rename = "small_file_id")]
    pub small: file::Id,
    /// The unique file ID of the small photo.
    #[serde(rename = "small_file_unique_id")]
    pub small_unique: String,
    /// The file ID of the big photo.
    #[serde(rename = "big_file_id")]
    pub big: file::Id,
    /// The unique file ID of the big photo.
    #[serde(rename = "big_file_unique_id")]
    pub big_unique: String,
}
