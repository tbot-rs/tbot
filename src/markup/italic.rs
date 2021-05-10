use super::{html, markdown_v2, Formattable, Nesting};
use std::fmt::{self, Formatter};

/// Formats text in italic. Can be created with [`italic`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use = "formatters need to be formatted with `markdown_v2` or `html`"]
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
            formatter.write_str("\r_")?;
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
            formatter.write_str("\r_")?;
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

#[cfg(tests)]
mod tests {
    use super::{italic, markdown_v2};
    #[test]
    fn sibling_italics_are_displayed_correctly() {
        assert_eq!(
            markdown_v2((italic("a"), italic("b"))).to_string(),
            "\r_a\r_\r_b\r_"
        );
    }
}
