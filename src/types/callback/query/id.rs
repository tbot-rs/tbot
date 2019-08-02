//! Types representing a callback query ID.

use serde::{Deserialize, Serialize};

/// Represents a callback query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);
