use crate::types::value;
use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub enum ParseMode {
    Markdown,
    #[serde(rename = "HTML")]
    Html,
}

/// Represents input text.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Text<'a> {
    pub(crate) text: value::String<'a>,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl Display for ParseMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            ParseMode::Markdown => "Markdown",
            ParseMode::Html => "HTML",
        })
    }
}

impl<'a> Text<'a> {
    /// Consructs new `Text` without any parse mode.
    pub fn plain(text: impl Into<value::String<'a>>) -> Self {
        Self {
            text: text.into(),
            parse_mode: None,
        }
    }

    /// Constructs new `Text` with `Markdown` parse mode.
    pub fn markdown(text: impl Into<value::String<'a>>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::Markdown),
        }
    }

    /// Constructs new `Text` with `HTML` parse mode.
    pub fn html(text: impl Into<value::String<'a>>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::Html),
        }
    }

    /// Checks if parse mode isn't set.
    pub fn is_plain(&self) -> bool {
        self.parse_mode == None
    }

    /// Checks if parse mode is `Markdown`.
    pub fn is_markdown(&self) -> bool {
        self.parse_mode == Some(ParseMode::Markdown)
    }

    /// Checks if parse mode is `Html`.
    pub fn is_html(&self) -> bool {
        self.parse_mode == Some(ParseMode::Html)
    }
}

impl<'a, T> From<T> for Text<'a>
where
    T: Into<value::String<'a>>,
{
    fn from(text: T) -> Self {
        Text::plain(text)
    }
}
