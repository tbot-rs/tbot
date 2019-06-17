use serde::*;

/// Represents a [`File`].
///
/// [`File`]: https://core.telegram.org/bots/api#file
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct File {
    /// The ID of the file.
    #[serde(rename = "file_id")]
    pub id: String,
    /// The size fo the file.
    #[serde(rename = "file_size")]
    pub size: Option<u32>,
    /// The path of the file.
    #[serde(rename = "file_path")]
    pub path: Option<String>,
}
