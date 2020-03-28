use super::{markdown_v2, Formattable};
use std::fmt::{self, Formatter, Write};

/// Formats text in bold. Can be created with [`bold`].
///
/// [`bold`]: ./fn.bold.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Bold<T>(T);

/// Formats text in bold.
pub fn bold<T: Formattable>(text: T) -> Bold<T> {
    Bold(text)
}

impl<T: Formattable> markdown_v2::Formattable for Bold<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_char('*')?;
        markdown_v2::Formattable::format(&self.0, formatter)?;
        formatter.write_char('*')
    }
}
