//! HTML markup utilities.

use super::Nesting;
use std::{
    fmt::{self, Display, Formatter, Write},
    ops::Deref,
};

/// Characters that need to be escaped to be interpreted as text.
pub const ESCAPED_TEXT_CHARACTERS: [(char, &str); 3] =
    [('<', "&lt;"), ('>', "&gt;"), ('&', "&amp;")];

/// Represents a value that can be formatted for HTML.
pub trait Formattable {
    /// Writes formatted value to the formatter.
    fn format(&self, formatter: &mut Formatter, _: Nesting) -> fmt::Result;
}

impl_primitives!(Formattable);
impl_tuples!(Formattable);

impl Formattable for char {
    fn format(&self, formatter: &mut Formatter, _: Nesting) -> fmt::Result {
        if let Some((_, escaped)) =
            ESCAPED_TEXT_CHARACTERS.iter().find(|(c, _)| *c == *self)
        {
            formatter.write_str(escaped)
        } else {
            formatter.write_char(*self)
        }
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

/// Represents HTML text. Can be created with [`html`].
///
/// [`html`]: ./fn.html.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use = "HTML needs to be turned into a String with `.to_string()`"]
pub struct Html<T>(T);

/// Creates HTML text.
pub fn html<T: Formattable>(content: T) -> Html<T> {
    Html(content)
}

impl<T: Formattable> Formattable for Html<T> {
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        self.0.format(formatter, nesting)
    }
}

impl<T: Formattable> Display for Html<T> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.format(formatter, Nesting::default())
    }
}
