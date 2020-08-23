//! Types related to passport element errors.

use crate::types::InteriorBorrow;
use serde::Serialize;
use std::borrow::Cow;

mod source;

pub use source::*;

/// Represents a [`PassportElementError`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerror
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Error<'a> {
    #[serde(flatten)]
    source: Source<'a>,
    message: Cow<'a, str>,
}

impl<'a> Error<'a> {
    /// Constructs a passport element `Error`.
    pub fn new(
        source: impl Into<Source<'a>>,
        message: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            source: source.into(),
            message: message.into(),
        }
    }
}

impl<'a> InteriorBorrow<'a> for Error<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            source: self.source.borrow_inside(),
            message: self.message.borrow_inside(),
        }
    }
}
