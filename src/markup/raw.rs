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
pub fn raw<I, T>(iterator: I) -> Raw<I>
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    T: Deref<Target = str>,
{
    Raw(iterator)
}

impl<I, T> markdown_v2::Formattable for Raw<I>
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    T: Deref<Target = str>,
{
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        (&self.0)
            .into_iter()
            .map(|x| formatter.write_str(&*x))
            .collect()
    }
}
