use super::{inline, reply, ForceReply};
use crate::types::InteriorBorrow;
use is_macro::Is;
use serde::Serialize;

/// An enum of possible keyboards.
#[derive(Serialize, Debug, PartialEq, Eq, Clone, Hash, Is)]
#[serde(untagged)]
#[non_exhaustive]
#[must_use]
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

impl<'a> From<inline::Markup<'a>> for Any<'a> {
    fn from(keyboard: inline::Markup<'a>) -> Self {
        Any::Inline(keyboard.into())
    }
}

impl<'a> From<reply::Keyboard<'a>> for Any<'a> {
    fn from(keyboard: reply::Keyboard<'a>) -> Self {
        Any::Reply(keyboard)
    }
}

impl<'a> From<reply::Markup<'a>> for Any<'a> {
    fn from(keyboard: reply::Markup<'a>) -> Self {
        Any::Reply(keyboard.into())
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

impl<'a> InteriorBorrow<'a> for Any<'a> {
    fn borrow_inside(&'a self) -> Self {
        match self {
            Self::Inline(inline) => Self::Inline(inline.borrow_inside()),
            Self::Reply(reply) => Self::Reply(reply.borrow_inside()),
            Self::RemoveReply(remove_reply) => Self::RemoveReply(*remove_reply),
            Self::ForceReply(force_reply) => Self::ForceReply(*force_reply),
        }
    }
}
