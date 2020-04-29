//! A few useful predicates for chats.

use crate::contexts::fields::Message;
use std::sync::Arc;

/// Checks if the message is from a private chat.
pub async fn is_private(context: Arc<impl Message>) -> bool {
    context.chat().kind.is_private()
}

/// Checks if the message is from a group.
pub async fn is_group(context: Arc<impl Message>) -> bool {
    context.chat().kind.is_group()
}

/// Checks if the message is from a supergroup.
pub async fn is_supergroup(context: Arc<impl Message>) -> bool {
    context.chat().kind.is_supergroup()
}

/// Checks if the message is from a channel.
pub async fn is_channel(context: Arc<impl Message>) -> bool {
    context.chat().kind.is_channel()
}
