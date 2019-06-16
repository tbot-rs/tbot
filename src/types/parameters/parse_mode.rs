use serde::Serialize;
use std::fmt::{self, Display, Formatter};

/// Represents the markup language of a message.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
// todo: #[non_exhaustive]
pub enum ParseMode {
    /// The message will be parsed as Markdown.
    Markdown,
    #[serde(rename = "HTML")]
    /// The message will be parsed as HTML.
    Html,
}

impl Display for ParseMode {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(match self {
            ParseMode::Markdown => "Markdown",
            ParseMode::Html => "HTML",
        })
    }
}
