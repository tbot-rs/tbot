use super::*;
use std::sync::Arc;

/// Context for the [`poll`] handler.
///
/// [`poll`]: ../struct.Bot.html#method.poll
#[derive(Clone)]
pub struct PollContext {
    /// A mock bot with all API methods.
    pub bot: Arc<MockBot>,
    /// Id of the message.
    pub message_id: u32,
    /// The sender of the message.
    pub from: Option<types::User>,
    /// The time the message was sent at.
    pub date: i64,
    /// The chat where the message was sent.
    pub chat: types::raw::Chat,
    /// The origin of the message if it is a forward.
    pub forward: Option<types::Forward>,
    /// If `Some`, the original message.
    pub reply_to: Option<types::Message>,
    /// The poll.
    pub poll: types::Poll,
}

impl PollContext {
    pub(crate) const fn new(
        bot: Arc<MockBot>,
        message_id: u32,
        from: Option<types::User>,
        date: i64,
        chat: types::raw::Chat,
        forward: Option<types::Forward>,
        reply_to: Option<types::Message>,
        poll: types::Poll,
    ) -> Self {
        Self {
            bot,
            message_id,
            from,
            date,
            chat,
            forward,
            reply_to,
            poll,
        }
    }
}

impl<'a> traits::ChatMethods<'a> for PollContext {
    fn bot(&self) -> &MockBot {
        &self.bot
    }

    fn chat_id(&self) -> i64 {
        self.chat.id
    }

    fn message_id(&self) -> u32 {
        self.message_id
    }
}

impl<'a> Forwardable<'a> for PollContext {}
