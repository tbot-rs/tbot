use super::{inline, reply, ForceReply};
use is_macro::Is;
use serde::Serialize;

/// An enum of possible keyboards.
#[derive(Serialize, Debug, PartialEq, Eq, Clone, Hash, Is)]
#[serde(untagged)]
#[non_exhaustive]
#[must_use]
pub enum Any {
    /// An inline keyboard.
    Inline(inline::Keyboard),
    /// A reply markup.
    Reply(reply::Keyboard),
    /// Removes reply markup.
    RemoveReply(reply::Remove),
    /// Forces reply.
    ForceReply(ForceReply),
}

impl From<inline::Keyboard> for Any {
    fn from(keyboard: inline::Keyboard) -> Self {
        Self::Inline(keyboard)
    }
}

impl From<inline::Markup> for Any {
    fn from(keyboard: inline::Markup) -> Self {
        Self::Inline(keyboard.into())
    }
}

impl From<reply::Keyboard> for Any {
    fn from(keyboard: reply::Keyboard) -> Self {
        Self::Reply(keyboard)
    }
}

impl From<reply::Markup> for Any {
    fn from(keyboard: reply::Markup) -> Self {
        Self::Reply(keyboard.into())
    }
}

impl From<reply::Remove> for Any {
    fn from(keyboard: reply::Remove) -> Self {
        Self::RemoveReply(keyboard)
    }
}

impl From<ForceReply> for Any {
    fn from(keyboard: ForceReply) -> Self {
        Self::ForceReply(keyboard)
    }
}
