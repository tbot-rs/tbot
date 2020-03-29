use super::{html, markdown_v2, Formattable, Nesting};
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
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        if !nesting.italic {
            formatter.write_char('_')?;
        }
        markdown_v2::Formattable::format(
            &self.0,
            formatter,
            Nesting {
                italic: true,
                ..nesting
            },
        )?;
        if !nesting.italic {
            formatter.write_char('_')?;
        }
        Ok(())
    }
}

impl<T: Formattable> html::Formattable for Italic<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_str("<i>")?;
        html::Formattable::format(&self.0, formatter, nesting)?;
        formatter.write_str("</i>")
    }
}
