use super::*;
use std::sync::Arc;

/// Context for the [`edited_text`] handler.
///
/// [`edited_text`]: ../struct.Bot.html#method.edited_text
#[derive(Clone)]
pub struct EditedTextContext {
    private: (),
    /// A mock bot with all API methods.
    pub bot: Arc<MockBot>,
    /// Id of the message.
    pub message_id: i32,
    /// The sender of the message.
    pub from: Option<types::User>,
    /// The time the message was sent at.
    pub date: i64,
    /// The chat where the message was sent.
    pub chat: types::raw::Chat,
    // No forward because one can't edit a forwared message.
    /// If `Some`, the original message.
    pub reply_to: Option<types::Message>,
    /// The message's text.
    pub text: String,
    /// Entities in the message (links, formatting, etc).
    pub entities: Vec<types::MessageEntity>,
    /// Last time when the message was edited.
    pub edit_date: i64,
}

impl EditedTextContext {
    pub(crate) fn new(
        bot: Arc<MockBot>,
        message_id: i32,
        from: Option<types::User>,
        date: i64,
        chat: types::raw::Chat,
        reply_to: Option<types::Message>,
        edit_date: i64,
        text: types::Text,
    ) -> Self {
        Self {
            private: (),
            bot,
            message_id,
            from,
            date,
            chat,
            reply_to,
            text: text.text,
            entities: text.entities,
            edit_date,
        }
    }
}

impl<'a> traits::ChatMethods<'a> for EditedTextContext {
    fn bot(&self) -> &MockBot {
        &self.bot
    }

    fn chat_id(&self) -> i64 {
        self.chat.id
    }

    fn message_id(&self) -> u64 {
        self.message_id as u64
    }
}

impl<'a> Forwardable<'a> for EditedTextContext {}
