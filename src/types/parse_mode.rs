use super::*;

/// Represents what markup the text is in.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub enum ParseMode {
    /// The message will be parsed as Markdown.
    Markdown,
    #[serde(rename = "HTML")]
    /// The message will be parsed as HTML.
    Html,
}
