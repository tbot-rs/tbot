//! Types representing a shipping query ID.

use serde::{Deserialize, Serialize};

/// Represents a shipping query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);
