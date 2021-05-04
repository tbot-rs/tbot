//! Types related to passport element errors.

use serde::Serialize;

mod source;

pub use source::*;

/// Represents a [`PassportElementError`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerror
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Error {
    #[serde(flatten)]
    source: Source,
    message: String,
}

impl Error {
    /// Constructs a passport element `Error`.
    pub fn new(
        source: impl Into<Source>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            message: message.into(),
        }
    }
}
