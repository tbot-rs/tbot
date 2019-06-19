//! Types related to passport element errors.

use serde::Serialize;

mod source;

pub use source::*;

/// Represents a [`PassportElementError`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerror
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Error<'a> {
    #[serde(flatten)]
    source: Source<'a>,
    message: &'a str,
}

impl<'a> Error<'a> {
    /// Constructs a passport element `Error`.
    pub fn new(source: Source<'a>, message: &'a str) -> Self {
        Self {
            source,
            message,
        }
    }
}
