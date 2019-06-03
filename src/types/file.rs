use serde::*;

/// Represents a [`File`].
///
/// [`File`]: https://core.telegram.org/bots/api#file
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct File {
    /// The ID of the file.
    pub file_id: String,
    /// The size fo the file.
    pub file_size: Option<u32>,
    /// The path of the file.
    pub file_path: Option<String>,
}
