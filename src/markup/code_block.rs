use super::{html, markdown_v2, Nesting};
use std::{
    fmt::{self, Formatter, Write},
    ops::Deref,
};

/// Formats a block of code. Can be created with [`code_block`].
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use = "formatters need to be formatted with `markdown_v2` or `html`"]
pub struct CodeBlock<C> {
    code: C,
    language: Option<String>,
}

impl<C> CodeBlock<C> {
    /// Defines the langauge of the code block.
    ///
    /// # Panics
    ///
    /// Panics if the language contains a line break or a quote.
    pub fn language(mut self, language: impl Into<String>) -> Self {
        let language = language.into();
        if language.contains('\n') {
            panic!(
                "[tbot] A code block's language may not contain line breaks: {}",
                &*language,
            );
        }

        if language.contains('"') {
            panic!(
                "[tbot] A code block's language may not contain quotes: {}",
                &*language,
            );
        }

        self.language = Some(language);
        self
    }
}

/// Formats a block of code.
pub fn code_block<C>(code: C) -> CodeBlock<C>
where
    C: Deref<Target = str>,
{
    CodeBlock {
        code,
        language: None,
    }
}

impl<C> markdown_v2::Formattable for CodeBlock<C>
where
    C: Deref<Target = str>,
{
    fn format(&self, formatter: &mut Formatter, _: Nesting) -> fmt::Result {
        formatter.write_str("```")?;
        if let Some(language) = &self.language {
            language.chars().try_for_each(|x| {
                if markdown_v2::ESCAPED_CODE_CHARACTERS.contains(&x) {
                    formatter.write_char('\\')?;
                }
                formatter.write_char(x)
            })?;
        }
        formatter.write_char('\n')?;

        self.code.chars().try_for_each(|x| {
            if markdown_v2::ESCAPED_CODE_CHARACTERS.contains(&x) {
                formatter.write_char('\\')?;
            }
            formatter.write_char(x)
        })?;
        formatter.write_str("\n```")
    }
}

impl<C> html::Formattable for CodeBlock<C>
where
    C: Deref<Target = str>,
{
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_str("<pre>")?;

        if let Some(language) = &self.language {
            write!(formatter, "<code class=\"language-{}\">", language)?;
        }

        html::Formattable::format(&&*self.code, formatter, nesting)?;

        if self.language.is_some() {
            formatter.write_str("</code>")?;
        }

        formatter.write_str("</pre>")
    }
}
