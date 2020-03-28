use super::{markdown_v2, Formattable};
use std::fmt::{self, Formatter, Write};

/// Formats text in italic. Can be created with [`italic`].
///
/// [`italic`]: ./fn.italic.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Italic<T>(T);

/// Formats text in italic.
pub fn italic<T: Formattable>(text: T) -> Italic<T> {
    Italic(text)
}

impl<T: Formattable> markdown_v2::Formattable for Italic<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_char('_')?;
        markdown_v2::Formattable::format(&self.0, formatter)?;
        formatter.write_char('_')
    }
}
