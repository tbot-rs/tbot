use super::*;
use std::sync::Arc;

/// Contants data for [text message handlers][on_message].
///
/// [on_message]: ../struct.Bot.html#method.on_message
#[derive(Clone)]
pub struct MessageContext {
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
    // TODO: implement `forward`
    /// If `Some`, the original message.
    pub reply_to: Option<Box<types::Message>>,
    /// If `Some`, the time the message was last edited at.
    pub edit_date: Option<i64>,
    /// The message's text.
    pub message: String,
    /// Entities in the message (links, formatting, etc).
    pub entities: Vec<types::MessageEntity>,
}

impl MessageContext {
    pub(crate) fn try_new(
        bot: Arc<MockBot>,
        message: types::Message,
    ) -> Result<Self, types::Message> {
        let text = match message.kind {
            types::MessageKind::Text(text) => text,
            _ => return Err(message),
        };

        Ok(Self {
            bot,
            message_id: message.id,
            from: message.from,
            date: message.date,
            chat: message.chat,
            reply_to: message.reply_to,
            edit_date: message.edit_date,
            message: text.text,
            entities: text.entities,
        })
    }
}

impl<'a> traits::ChatMethods<'a> for MessageContext {
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

impl<'a> Forwardable<'a> for MessageContext {}
