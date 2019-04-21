use super::*;

/// Represents a [`Document`].
///
/// [`Document`]: https://core.telegram.org/bots/api#document
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Document {
    /// The file ID of the document.
    pub file_id: String,
    /// The thumb of the document.
    pub thumb: Option<PhotoSize>,
    /// The document file's name.
    pub file_name: Option<String>,
    /// The MIME type of the document.
    pub mime_type: Option<String>,
    /// The document file's size.
    pub file_size: Option<u32>,
}
