use super::*;

type Bot = std::sync::Arc<MockBot>;

/// Contants data for [text message handlers][on_message].
///
/// [on_message]: ../struct.Bot.html#method.on_message
pub struct MessageContext {
    /// A mock bot with all API methods.
    pub bot: Bot,
    /// Id of the message.
    pub message_id: u64,
    /// The sender of the message.
    pub from: types::User,
    /// The time the message was sent at.
    pub date: i64,
    /// The chat where the message was sent.
    pub chat: types::raw::Chat,
    // TODO: implement `forward`
    /// If `Some`, the original message.
    pub reply_to: Option<Box<types::raw::Message>>,
    /// If `Some`, the time the message was last edited at.
    pub edit_date: Option<i64>,
    /// The message's text.
    pub message: String,
    /// Entities in the message (links, formatting, etc).
    pub entities: Vec<types::raw::MessageEntity>,
}

impl MessageContext {
    pub(crate) fn new(bot: Bot, message: types::raw::Message) -> Option<Self> {
        Some(Self {
            bot,
            message_id: message.message_id,
            from: message.from,
            date: message.date,
            chat: message.chat,
            reply_to: message.reply_to_message,
            edit_date: message.edit_date,
            message: message.text?,
            entities: message.entities.unwrap_or_else(|| Vec::new()),
        })
    }
}

impl traits::ChatMethods for MessageContext {
    fn bot<'a>(&'a self) -> &'a MockBot {
        &self.bot
    }

    fn chat_id(&self) -> i64 {
        self.chat.id
    }

    fn message_id(&self) -> u64 {
        self.message_id
    }

    fn from_id(&self) -> i64 {
        self.from.id
    }
}
