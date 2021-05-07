//! Types representing a file ID.

use serde::{Deserialize, Serialize};

/// Represents a file ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

impl From<String> for Id {
    #[must_use]
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl<'a> From<&'a String> for Id {
    #[must_use]
    fn from(id: &'a String) -> Self {
        Self(id.clone())
    }
}

impl<'a> From<&'a str> for Id {
    #[must_use]
    fn from(id: &'a str) -> Self {
        Self(id.to_owned())
    }
}
