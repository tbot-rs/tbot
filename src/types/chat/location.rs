use crate::types::location;
use serde::Deserialize;

/// Represents a location to which a chat is connected.
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[non_exhaustive]
pub struct Location {
    /// The location to which the supergroup is connected.
    /// Can't be a live location.
    pub location: location::Location,
    /// Location address; 1-64 characters, as defined by the chat owner.
    pub address: String,
}
