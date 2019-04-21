use serde::*;

/// Represents a [`File`].
///
/// [`File`]: https://core.telegram.org/bots/api#file
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct File {
    /// The file's ID.
    pub file_id: String,
    /// The file's size.
    pub file_size: Option<u32>,
    /// The file's name.
    pub file_path: Option<String>,
}
