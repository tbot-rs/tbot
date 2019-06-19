use serde::Deserialize;

/// Represents a [`PassportFile`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportfile
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct File {
    /// The ID of the file.
    pub id: String,
    /// The size of the file.
    pub size: usize,
    /// The date of the file.
    pub date: i64,
}
