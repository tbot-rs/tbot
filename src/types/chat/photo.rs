use crate::types::file;
use serde::Deserialize;

/// Represents a [`ChatPhoto`].
///
/// [`ChatPhoto`]: https://core.telegram.org/bots/api#chatphoto
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct Photo {
    /// The file ID of the small photo.
    #[serde(rename = "small_file_id")]
    pub small: file::Id,
    /// The file ID of the big photo.
    #[serde(rename = "big_file_id")]
    pub big: file::Id,
}
