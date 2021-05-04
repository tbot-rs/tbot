//! Types representing an inline message ID.

use serde::{Deserialize, Serialize};

/// Represents an inline message ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InlineMessageId(pub String);

impl From<String> for InlineMessageId {
    #[must_use]
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl<'a> From<&'a String> for InlineMessageId {
    #[must_use]
    fn from(id: &'a String) -> Self {
        Self(id.to_owned())
    }
}

impl<'a> From<&'a str> for InlineMessageId {
    #[must_use]
    fn from(id: &'a str) -> Self {
        Self(id.to_owned())
    }
}
