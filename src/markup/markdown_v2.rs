//! MarkdownV2 markup utilities.

use super::Nesting;
use crate::types::parameters::Text;
use std::{
    fmt::{self, Display, Formatter, Write},
    ops::Deref,
};

/// Characters that need to be escaped to be interpreted as text.
pub const ESCAPED_TEXT_CHARACTERS: [char; 19] = [
    '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{',
    '}', '.', '!', '\\',
];

/// Characters that need to be escaped to be interpreted as code.
pub const ESCAPED_CODE_CHARACTERS: [char; 2] = ['`', '\\'];

/// Characters that need to be escaped to be interpreted as a link.
pub const ESCAPED_LINK_CHARACTERS: [char; 2] = [')', '\\'];

/// Represents a value that can be formatted for MarkdownV2.
pub trait Formattable {
    // This is not meant to be public, thus relying on it may break you code
    // at any time
    #[doc(hidden)]
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result;
}

impl_primitives!(Formattable);
impl_tuples!(Formattable);

impl Formattable for char {
    fn format(&self, formatter: &mut Formatter, _: Nesting) -> fmt::Result {
        if ESCAPED_TEXT_CHARACTERS.contains(self) {
            formatter.write_char('\\')?;
        }

        formatter.write_char(*self)
    }
}

impl Formattable for &'_ str {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        self.chars()
            .map(|character| character.format(formatter, nesting))
            .collect()
    }
}

impl Formattable for String {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        self.as_str().format(formatter, nesting)
    }
}

impl<T: Formattable> Formattable for &'_ [T] {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        self.iter().map(|x| x.format(formatter, nesting)).collect()
    }
}

impl<T: Formattable> Formattable for Vec<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        self.as_slice().format(formatter, nesting)
    }
}

impl<T: Formattable + ?Sized> Formattable for Box<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        self.deref().format(formatter, nesting)
    }
}

/// Represents MarkdownV2 text. Can be created with [`markdown_v2`].
///
/// [`markdown_v2`]: ./fn.markdown_v2.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use = "MarkdownV2 needs to be turned into a `Text` instance"]
pub struct MarkdownV2<T>(T);

struct Displayable<T>(MarkdownV2<T>);

/// Creates MarkdownV2 text.
pub fn markdown_v2<T: Formattable>(content: T) -> MarkdownV2<T> {
    MarkdownV2(content)
}

impl<T: Formattable> Formattable for MarkdownV2<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        self.0.format(formatter, nesting)
    }
}

impl<T: Formattable> Display for Displayable<T> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.format(formatter, Nesting::default())
    }
}

impl<T: Formattable> From<MarkdownV2<T>> for Text<'_> {
    fn from(markup: MarkdownV2<T>) -> Self {
        let message = Displayable(markup).to_string();
        Text::with_markdown_v2(message)
    }
}
