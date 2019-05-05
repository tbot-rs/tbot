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
    /// The author's signature, if turned for the channel.
    pub author_signature: Option<String>,
    /// The poll.
    pub poll: types::Poll,
}

impl PollContext {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: Arc<MockBot>,
        data: types::MessageData,
        poll: types::Poll,
    ) -> Self {
        Self {
            bot,
            message_id: data.id,
            from: data.from,
            date: data.date,
            chat: data.chat,
            forward: data.forward,
            reply_to: data.reply_to,
            author_signature: data.author_signature,
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
