//! Types representing a shipping query ID.

use serde::{Deserialize, Serialize};

/// Represents a shipping query ID.
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
        Self(id.to_owned())
    }
}

impl<'a> From<&'a str> for Id {
    #[must_use]
    fn from(id: &'a str) -> Self {
        Self(id.to_owned())
    }
}
