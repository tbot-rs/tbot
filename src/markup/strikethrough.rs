use super::{html, markdown_v2, Formattable, Nesting};
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
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        if !nesting.strikethrough {
            formatter.write_char('~')?;
        }
        markdown_v2::Formattable::format(
            &self.0,
            formatter,
            Nesting {
                strikethrough: true,
                ..nesting
            },
        )?;
        if !nesting.strikethrough {
            formatter.write_char('~')?;
        }
        Ok(())
    }
}

impl<T: Formattable> html::Formattable for Strikethrough<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_str("<s>")?;
        html::Formattable::format(&self.0, formatter, nesting)?;
        formatter.write_str("</s>")
    }
}
