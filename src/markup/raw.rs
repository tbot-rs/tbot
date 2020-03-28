use super::markdown_v2;
use std::{
    fmt::{self, Formatter},
    ops::Deref,
};

/// Represents a raw string for formatting. Can be created with [`raw`].
///
/// [`raw`]: ./fn.raw.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Raw<T>(T);

/// Creates a raw string for formatting.
///
/// **Use this utility with extreme care**: it inserts the provided string into
/// the resulting string as-is. As a result, if it contains malformed
/// formatting, the resulting string won't be parsed by Telegram. Also,
/// unchecked user-provided input may insert its own formatting, which may be
/// undesrirable. Note that all other utilities automatically escape provided
/// strings as needed.
pub fn raw<T: Deref<Target = str>>(string: T) -> Raw<T> {
    Raw(string)
}

impl<T: Deref<Target = str>> markdown_v2::Formattable for Raw<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&*self.0)
    }
}
