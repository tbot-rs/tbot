//! Types representing a file ID.

use crate::internal::Sealed;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a file ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id<'a>(pub Cow<'a, str>);

impl<'a> Id<'a> {
    /// Create a new reference to a file ID.
    #[must_use]
    pub fn as_ref(&'a self) -> Self {
        Self(Cow::Borrowed(&self.0))
    }
}

impl<'a> From<String> for Id<'a> {
    #[must_use]
    fn from(id: String) -> Self {
        Self(id.into())
    }
}

impl<'a> From<&'a str> for Id<'a> {
    #[must_use]
    fn from(id: &'a str) -> Self {
        Self(id.into())
    }
}

impl<'a> Sealed for Id<'a> {}

/// Allows a type with a unique file ID to act as [`file::Id`].
///
/// [`file::Id`]: ./struct.Id.html
#[allow(clippy::module_name_repetitions)] // can't think of a better name
pub trait AsFileId<'a>: Sealed {
    #[doc(hidden)]
    fn as_file_id(&'a self) -> Id<'a>;
}

impl<'a> AsFileId<'a> for Id<'a> {
    #[must_use]
    fn as_file_id(&self) -> Id<'_> {
        self.as_ref()
    }
}
