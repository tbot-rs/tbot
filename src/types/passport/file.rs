use crate::types::file;
use serde::Deserialize;

/// Represents a [`PassportFile`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportfile
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct File {
    /// The ID of the file.
    pub id: file::Id,
    /// The unique ID of the file.
    pub unique_id: String,
    /// The size of the file.
    pub size: usize,
    /// The date of the file.
    pub date: i64,
}
