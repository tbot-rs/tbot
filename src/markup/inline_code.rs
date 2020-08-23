use super::{html, markdown_v2, Nesting};
use std::{
    fmt::{self, Formatter, Write},
    ops::Deref,
};

/// Formats an inline piece of code. Can be created with [`inline_code`].
///
/// [`inline_code`]: ./fn.inline_code.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use = "formatters need to be formatted with `markdown_v2` or `html`"]
pub struct InlineCode<T>(T);

/// Formats an inline piece of code.
pub fn inline_code<T>(code: T) -> InlineCode<T>
where
    T: Deref<Target = str>,
{
    InlineCode(code)
}

impl<T> markdown_v2::Formattable for InlineCode<T>
where
    T: Deref<Target = str>,
{
    fn format(&self, formatter: &mut Formatter, _: Nesting) -> fmt::Result {
        formatter.write_char('`')?;
        self.0
            .chars()
            .map(|x| {
                if markdown_v2::ESCAPED_CODE_CHARACTERS.contains(&x) {
                    formatter.write_char('\\')?;
                }
                formatter.write_char(x)
            })
            .collect::<Result<(), _>>()?;
        formatter.write_char('`')
    }
}

impl<T> html::Formattable for InlineCode<T>
where
    T: Deref<Target = str>,
{
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_str("<code>")?;
        html::Formattable::format(&&*self.0, formatter, nesting)?;
        formatter.write_str("</code>")
    }
}
