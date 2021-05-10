use super::{html, markdown_v2, Formattable, Nesting};
use std::fmt::{self, Formatter};

/// Formats text underlined. Can be created with [`underline`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use = "formatters need to be formatted with `markdown_v2` or `html`"]
pub struct Underline<T>(T);

/// Formats text underlined.
pub fn underline<T: Formattable>(text: T) -> Underline<T> {
    Underline(text)
}

impl<T: Formattable> markdown_v2::Formattable for Underline<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        if !nesting.underline {
            formatter.write_str("\r__")?;
        }
        markdown_v2::Formattable::format(
            &self.0,
            formatter,
            Nesting {
                underline: true,
                ..nesting
            },
        )?;
        if !nesting.underline {
            formatter.write_str("\r__")?;
        }
        Ok(())
    }
}

impl<T: Formattable> html::Formattable for Underline<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_str("<u>")?;
        html::Formattable::format(&self.0, formatter, nesting)?;
        formatter.write_str("</u>")
    }
}
