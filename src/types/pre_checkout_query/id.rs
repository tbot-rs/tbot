//! Types representing a pre-checkout query ID.

use serde::{Deserialize, Serialize};

/// Represents a pre-checkout query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);
