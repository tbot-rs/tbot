//! Types representing a pre-checkout query ID.

use serde::{Deserialize, Serialize};

/// Represents a pre-checkout query ID.
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
        Self(id.into())
    }
}

impl<'a> From<&'a str> for Id {
    #[must_use]
    fn from(id: &'a str) -> Self {
        Self(id.into())
    }
}
