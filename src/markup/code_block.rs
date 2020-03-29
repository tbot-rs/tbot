use super::markdown_v2;
use std::{
    fmt::{self, Formatter, Write},
    ops::Deref,
};

/// Formats a block of code. Can be created with [`code_block`].
///
/// [`code_block`]: ./fn.code_block.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CodeBlock<C, L = &'static str> {
    code: C,
    language: Option<L>,
}

impl<C, L: Deref<Target = str>> CodeBlock<C, L> {
    /// Defines the langauge of the code block.
    ///
    /// # Panics
    ///
    /// Panics if the language contains a line break.
    pub fn language(mut self, language: L) -> Self {
        if language.deref().contains('\n') {
            panic!(
                "[tbot] A code block's language may not contain line breaks: {}",
                language.deref(),
            );
        }

        self.language = Some(language);
        self
    }
}

/// Formats a block of code.
pub fn code_block<I, T, L>(code: I) -> CodeBlock<I, L>
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    T: Deref<Target = str>,
    L: Deref<Target = str>,
{
    CodeBlock {
        code,
        language: None,
    }
}

impl<I, T, L> markdown_v2::Formattable for CodeBlock<I, L>
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    T: Deref<Target = str>,
    L: Deref<Target = str>,
{
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("```")?;
        if let Some(language) = &self.language {
            language
                .deref()
                .chars()
                .map(|x| {
                    if markdown_v2::ESCAPED_CODE_ENTITIES.contains(&x) {
                        formatter.write_char('\\')?;
                    }
                    formatter.write_char(x)
                })
                .collect::<Result<(), _>>()?;
        }
        formatter.write_char('\n')?;

        (&self.code)
            .into_iter()
            .flat_map(|x| x.deref().chars())
            .map(|x| {
                if markdown_v2::ESCAPED_CODE_ENTITIES.contains(&x) {
                    formatter.write_char('\\')?;
                }
                formatter.write_char(x)
            })
            .collect::<Result<(), _>>()?;
        formatter.write_str("\n```")
    }
}
