use super::{markdown_v2, Formattable};
use std::fmt::{self, Formatter, Write};

/// Formats text with strikethrough. Can be created with [`strikethrough`].
///
/// [`strikethrough`]: ./fn.strikethrough.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Strikethrough<T>(T);

/// Formats text with strikethrough.
pub fn strikethrough<T: Formattable>(text: T) -> Strikethrough<T> {
    Strikethrough(text)
}

impl<T: Formattable> markdown_v2::Formattable for Strikethrough<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_char('~')?;
        markdown_v2::Formattable::format(&self.0, formatter)?;
        formatter.write_char('~')
    }
}
