use super::{inline, reply, ForceReply};
use serde::Serialize;

/// An enum of possible keyboards.
#[derive(Serialize, Debug, PartialEq, Eq, Clone, Hash)]
#[serde(untagged)]
// todo: #[non_exhaustive]
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

impl Any<'_> {
    /// Checks if `self` is `Inline`.
    pub fn is_inline(&self) -> bool {
        match self {
            Any::Inline(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Reply`.
    pub fn is_reply(&self) -> bool {
        match self {
            Any::Reply(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `RemoveReply`.
    pub fn is_remove_reply(&self) -> bool {
        match self {
            Any::RemoveReply(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `ForceReply`.
    pub fn is_force_reply(&self) -> bool {
        match self {
            Any::ForceReply(..) => true,
            _ => false,
        }
    }
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

impl<'a> From<Vec<inline::InnerMarkup<'a>>> for Any<'a> {
    fn from(keyboard: Vec<inline::InnerMarkup<'a>>) -> Self {
        Any::Inline(keyboard.into())
    }
}

impl<'a> From<&'a Vec<inline::InnerMarkup<'a>>> for Any<'a> {
    fn from(keyboard: &'a Vec<inline::InnerMarkup<'a>>) -> Self {
        Any::Inline(keyboard.into())
    }
}

impl<'a> From<&'a [inline::InnerMarkup<'a>]> for Any<'a> {
    fn from(keyboard: &'a [inline::InnerMarkup<'a>]) -> Self {
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

impl<'a> From<Vec<reply::InnerMarkup<'a>>> for Any<'a> {
    fn from(keyboard: Vec<reply::InnerMarkup<'a>>) -> Self {
        Any::Reply(keyboard.into())
    }
}

impl<'a> From<&'a Vec<reply::InnerMarkup<'a>>> for Any<'a> {
    fn from(keyboard: &'a Vec<reply::InnerMarkup<'a>>) -> Self {
        Any::Reply(keyboard.into())
    }
}

impl<'a> From<&'a [reply::InnerMarkup<'a>]> for Any<'a> {
    fn from(keyboard: &'a [reply::InnerMarkup<'a>]) -> Self {
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
