use super::ParseMode;

/// Represents input text.
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
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(text: &'a str) -> Self {
        Text::plain(text)
    }
}
