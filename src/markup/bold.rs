use super::{html, markdown_v2, Formattable, Nesting};
use std::fmt::{self, Formatter, Write};

/// Formats text in bold. Can be created with [`bold`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use = "formatters need to be formatted with `markdown_v2` or `html`"]
pub struct Bold<T>(T);

/// Formats text in bold.
pub fn bold<T: Formattable>(text: T) -> Bold<T> {
    Bold(text)
}

impl<T: Formattable> markdown_v2::Formattable for Bold<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        if !nesting.bold {
            formatter.write_char('*')?;
        }
        markdown_v2::Formattable::format(
            &self.0,
            formatter,
            Nesting {
                bold: true,
                ..nesting
            },
        )?;
        if !nesting.bold {
            formatter.write_char('*')?;
        }
        Ok(())
    }
}

impl<T: Formattable> html::Formattable for Bold<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_str("<b>")?;
        html::Formattable::format(&self.0, formatter, nesting)?;
        formatter.write_str("</b>")
    }
}
