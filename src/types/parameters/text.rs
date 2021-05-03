use is_macro::Is;
use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[must_use]
pub enum ParseMode {
    MarkdownV2,
    Markdown,
    #[serde(rename = "HTML")]
    Html,
}

/// Represents input text.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Text {
    pub(crate) text: String,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl Display for ParseMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MarkdownV2 => "MarkdownV2",
            Self::Markdown => "Markdown",
            Self::Html => "HTML",
        })
    }
}

impl Text {
    /// Constructs new `Text` without any parse mode.
    pub fn with_plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: None,
        }
    }

    /// Constructs new `Text` with `Markdown` parse mode.
    pub fn with_markdown(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::Markdown),
        }
    }

    /// Constructs new `Text` with `MarkdownV2` parse mode.
    pub fn with_markdown_v2(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::MarkdownV2),
        }
    }

    /// Constructs new `Text` with `HTML` parse mode.
    pub fn with_html(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::Html),
        }
    }

    /// Checks if parse mode isn't set.
    #[must_use]
    pub fn is_plain(&self) -> bool {
        self.parse_mode == None
    }

    /// Checks if parse mode is `MarkdownV2`.
    #[must_use]
    pub fn is_markdown_v2(&self) -> bool {
        self.parse_mode == Some(ParseMode::MarkdownV2)
    }

    /// Checks if parse mode is `Markdown`.
    #[must_use]
    pub fn is_markdown(&self) -> bool {
        self.parse_mode == Some(ParseMode::Markdown)
    }

    /// Checks if parse mode is `Html`.
    #[must_use]
    pub fn is_html(&self) -> bool {
        self.parse_mode == Some(ParseMode::Html)
    }
}

impl<T: Into<String>> From<T> for Text {
    fn from(text: T) -> Self {
        Self::with_plain(text)
    }
}
