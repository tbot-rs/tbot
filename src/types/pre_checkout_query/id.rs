//! Types representing a pre-checkout query ID.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a pre-checkout query ID.
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
