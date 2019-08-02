//! Types representing an inline query ID.

use serde::{Deserialize, Serialize};

/// Represents an inline query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);
