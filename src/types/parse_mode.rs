use super::*;

/// Represents the markup language of a message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub enum ParseMode {
    /// The message will be parsed as Markdown.
    Markdown,
    #[serde(rename = "HTML")]
    /// The message will be parsed as HTML.
    Html,
}

impl std::fmt::Display for ParseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            ParseMode::Markdown => "Markdown",
            ParseMode::Html => "HTML",
        })
    }
}
