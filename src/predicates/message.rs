//! A few useful predicates for messages.

use crate::contexts::fields::{Forward, MediaMessage};
use std::sync::Arc;

/// Checks if the message replies to another message.
pub async fn is_in_reply(context: Arc<impl MediaMessage>) -> bool {
    context.reply_to().is_some()
}

/// Checks if the message is forwarded.
pub async fn is_forwarded(context: Arc<impl Forward>) -> bool {
    context.forward().is_some()
}
