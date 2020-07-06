//! Types related to inline queries.

use crate::types::{Location, User};
use serde::Deserialize;

mod id;
pub mod result;

pub use {id::Id, result::Result};

/// Represents an [`InlineQuery`].
///
/// [`InlineQuery`]: https://core.telegram.org/bots/api#inlinequery
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[non_exhaustive]
pub struct InlineQuery {
    /// The ID of the query.
    pub id: Id<'static>,
    /// The user who sent the query.
    pub from: User,
    /// The location of the user, if enabled and allowed.
    pub location: Option<Location>,
    /// The query itself.
    pub query: String,
    /// The offset of the result to be returned.
    pub offset: String,
}
