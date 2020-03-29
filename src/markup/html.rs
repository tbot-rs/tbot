//! HTML markup utilities.

use std::{
    fmt::{self, Display, Formatter, Write},
    ops::Deref,
};

/// Characters that need to be escaped to be interpreted as text.
pub const ESCAPED_TEXT_CHARACTERS: [(char, &str); 3] =
    [('<', "&lt;"), ('>', "&gt;"), ('&', "&amp;")];

/// Represents a value that can be formatted for HTML.
pub trait Formattable {
    /// Writes formatted value to the formatter.
    #[allow(clippy::missing_errors_doc)]
    fn format(&self, formatter: &mut Formatter) -> fmt::Result;
}

impl Formattable for str {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        self.chars()
            .map(|character| {
                if let Some((_, escaped)) = ESCAPED_TEXT_CHARACTERS
                    .iter()
                    .find(|(c, _)| *c == character)
                {
                    formatter.write_str(escaped)
                } else {
                    formatter.write_char(character)
                }
            })
            .collect()
    }
}

impl<T: Formattable> Formattable for [T] {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        self.iter().map(|x| x.format(formatter)).collect()
    }
}

impl<T> Formattable for T
where
    T: Deref,
    T::Target: Formattable,
{
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        self.deref().format(formatter)
    }
}

/// Represents Html text. Can be created with [`html`].
///
/// [`html`]: ./fn.html.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Html<T>(T);

/// Creates MarkdownV2 text.
pub fn html<T: Formattable>(content: T) -> Html<T> {
    Html(content)
}

impl<T: Formattable> Formattable for Html<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.format(formatter)
    }
}

impl<T: Formattable> Display for Html<T> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.format(formatter)
    }
}
