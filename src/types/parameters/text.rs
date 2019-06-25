use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub enum ParseMode {
    Markdown,
    #[serde(rename = "HTML")]
    Html,
}

/// Represents input text.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Text<'a> {
    pub(crate) text: &'a str,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl<'a> Text<'a> {
    /// Consructs new `Text` without any parse mode.
    pub const fn plain(text: &'a str) -> Self {
        Self {
            text,
            parse_mode: None,
        }
    }

    /// Constructs new `Text` with `Markdown` parse mode.
    pub fn markdown(text: &'a str) -> Self {
        Self {
            text,
            parse_mode: Some(ParseMode::Markdown),
        }
    }

    /// Constructs new `Text` with `HTML` parse mode.
    pub fn html(text: &'a str) -> Self {
        Self {
            text,
            parse_mode: Some(ParseMode::Html),
        }
    }

    /// Checks if parse mode isn't set.
    pub fn is_plain(self) -> bool {
        self.parse_mode == None
    }

    /// Checks if parse mode is `Markdown`.
    pub fn is_markdown(self) -> bool {
        self.parse_mode == Some(ParseMode::Markdown)
    }

    /// Checks if parse mode is `Html`.
    pub fn is_html(self) -> bool {
        self.parse_mode == Some(ParseMode::Html)
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(text: &'a str) -> Self {
        Text::plain(text)
    }
}

impl<'a> From<&'a String> for Text<'a> {
    fn from(text: &'a String) -> Self {
        Text::plain(text.as_str())
    }
}
