use super::{inline, reply, ForceReply};
use serde::Serialize;

/// An enum of possible keyboards.
#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[serde(untagged)]
pub enum Any<'a> {
    /// An inline keyboard.
    Inline(inline::Keyboard<'a>),
    /// A reply markup.
    Reply(reply::Keyboard<'a>),
    /// Removes reply markup.
    RemoveReply(reply::Remove),
    /// Forces reply.
    ForceReply(ForceReply),
}

impl<'a> From<inline::Keyboard<'a>> for Any<'a> {
    fn from(keyboard: inline::Keyboard<'a>) -> Self {
        Any::Inline(keyboard)
    }
}

impl<'a> From<reply::Keyboard<'a>> for Any<'a> {
    fn from(keyboard: reply::Keyboard<'a>) -> Self {
        Any::Reply(keyboard)
    }
}

impl<'a> From<reply::Remove> for Any<'a> {
    fn from(keyboard: reply::Remove) -> Self {
        Any::RemoveReply(keyboard)
    }
}

impl<'a> From<ForceReply> for Any<'a> {
    fn from(keyboard: ForceReply) -> Self {
        Any::ForceReply(keyboard)
    }
}
