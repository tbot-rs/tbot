//! MarkdownV2 markup utilities.

use std::{
    fmt::{self, Display, Formatter, Write},
    ops::Deref,
};

/// Characters that need to be escaped to be interpreted as text.
pub const ESCAPED_TEXT_CHARACTERS: [char; 19] = [
    '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{',
    '}', '.', '!', '\\',
];

/// Characters that need to be escaped to be interpreted as code.
pub const ESCAPED_CODE_CHARACTERS: [char; 2] = ['`', '\\'];

/// Characters that need to be escaped to be interpreted as a link.
pub const ESCAPED_LINK_CHARACTERS: [char; 2] = [')', '\\'];

/// Represents a value that can be formatted for MarkdownV2.
pub trait Formattable {
    /// Writes formatted value to the formatter.
    #[allow(clippy::missing_errors_doc)]
    fn format(&self, formatter: &mut Formatter) -> fmt::Result;
}

impl Formattable for str {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        self.chars()
            .map(|character| {
                if ESCAPED_TEXT_CHARACTERS.contains(&character) {
                    formatter.write_char('\\')?;
                }

                formatter.write_char(character)
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

/// Represents MarkdownV2 text. Can be created with [`markdown_v2`].
///
/// [`markdown_v2`]: ./fn.markdown_v2.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MarkdownV2<T>(T);

/// Creates MarkdownV2 text.
pub fn markdown_v2<T: Formattable>(content: T) -> MarkdownV2<T> {
    MarkdownV2(content)
}

impl<T: Formattable> Formattable for MarkdownV2<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.format(formatter)
    }
}

impl<T: Formattable> Display for MarkdownV2<T> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.format(formatter)
    }
}
