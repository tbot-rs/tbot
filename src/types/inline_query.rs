//! Types related to inline queries.

use crate::types::{Location, User};
use serde::Deserialize;
use is_macro::Is;

mod id;
pub mod result;

pub use {id::Id, result::Result};

/// Represents the kind of a chat.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Is, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatKind {
    /// The chat is private.
    Sender,
    /// The chat is private.
    Private,
    /// The chat is a channel.
    Channel,
    /// The chat is a group.
    Group,
    /// The chat is a supergroup.
    Supergroup,
}

/// Represents an [`InlineQuery`].
///
/// [`InlineQuery`]: https://core.telegram.org/bots/api#inlinequery
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[non_exhaustive]
pub struct InlineQuery {
    /// The ID of the query.
    pub id: Id,
    /// The user who sent the query.
    pub from: User,
    /// The location of the user, if enabled and allowed.
    pub location: Option<Location>,
    /// The query itself.
    pub query: String,
    /// The offset of the result to be returned.
    pub offset: String,
    /// The type of chat inline query was sent from.
    pub chat_type: Option<ChatKind>,
}
