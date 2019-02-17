use super::*;

/// Represents [`PhotoSize`].
///
/// [`PhotoSize`]: https://core.telegram.org/bots/api#photosize
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct PhotoSize {
    /// Photo's file ID.
    pub file_id: String,
    /// Photo's width.
    pub width: u64,
    /// Photo's height.
    pub height: u64,
    /// Photo's file size.
    pub file_size: Option<u64>,
}
