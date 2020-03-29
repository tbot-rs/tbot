use super::markdown_v2;
use std::{
    fmt::{self, Formatter, Write},
    ops::Deref,
};

/// Formats an inline piece of code. Can be created with [`inline_code`].
///
/// [`inline_code`]: ./fn.inline_code.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InlineCode<T>(T);

/// Formats an inline piece of code.
pub fn inline_code<I, T>(code: I) -> InlineCode<I>
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    T: Deref<Target = str>,
{
    InlineCode(code)
}

impl<I, T> markdown_v2::Formattable for InlineCode<I>
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    T: Deref<Target = str>,
{
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_char('`')?;
        (&self.0)
            .into_iter()
            .flat_map(|x| x.deref().chars())
            .map(|x| {
                if markdown_v2::ESCAPED_CODE_ENTITIES.contains(&x) {
                    formatter.write_char('\\')?;
                }
                formatter.write_char(x)
            })
            .collect::<Result<(), _>>()?;
        formatter.write_char('`')
    }
}
