use super::Context;
use crate::types::{callback, User};

/// A general trait for callback updates.
pub trait Callback: Context {
    /// The ID of the callback.
    fn id(&self) -> &callback::query::Id;
    /// The user who initiated the callback.
    fn from(&self) -> &User;
    /// The origin of the query.
    fn origin(&self) -> &callback::Origin;
    /// The identifier of the chat.
    fn chat_instance(&self) -> &str;
}
