use super::*;
use std::sync::Arc;

/// Context for the [`photo`] handler.
///
/// [`photo`]: ../struct.Bot.html#method.photo
#[derive(Clone)]
pub struct PhotoContext {
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
    /// The photo.
    pub photo: Vec<types::PhotoSize>,
    /// The photo's caption.
    pub caption: String,
    /// Entities in the caption (links, formatting, etc).
    pub caption_entities: Vec<types::MessageEntity>,
    /// The media group's ID.
    pub media_group_id: Option<i32>,
}

impl PhotoContext {
    pub(crate) fn new(
        bot: Arc<MockBot>,
        data: types::MessageData,
        photo: Vec<types::PhotoSize>,
        caption: types::Text,
        media_group_id: Option<i32>,
    ) -> Self {
        Self {
            bot,
            message_id: data.id,
            from: data.from,
            date: data.date,
            chat: data.chat,
            forward: data.forward,
            reply_to: data.reply_to.map(|message| *message),
            author_signature: data.author_signature,
            photo,
            caption: caption.text,
            caption_entities: caption.entities,
            media_group_id,
        }
    }
}

impl<'a> traits::ChatMethods<'a> for PhotoContext {
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

impl<'a> Forwardable<'a> for PhotoContext {}
