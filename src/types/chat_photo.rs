use super::*;

/// Represents a [`ChatPhoto`].
///
/// [`ChatPhoto`]: https://core.telegram.org/bots/api#chatphoto
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ChatPhoto {
    /// Small (160×160) photo's File ID.
    pub small_file_id: String,
    /// Big (640×640) photo's File ID.
    pub big_file_id: String,
}
