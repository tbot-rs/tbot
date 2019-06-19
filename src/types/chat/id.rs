use crate::types::user;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Represents a chat ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Id(pub i64);

impl From<i64> for Id {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl Display for Id {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<user::Id> for Id {
    fn from(id: user::Id) -> Self {
        Id(id.0)
    }
}
