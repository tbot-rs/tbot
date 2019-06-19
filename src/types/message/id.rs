use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Represents a message ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Id(pub u32);

impl From<u32> for Id {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl Display for Id {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
