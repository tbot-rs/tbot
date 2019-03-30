use super::*;

/// Contains methods for forwardable messages.
pub trait Forwardable<'a>: ChatMethods<'a> {
    /// Constructs a [`ForwardMessage`] inferring the token, chat ID and
    /// message ID.
    ///
    /// [`ForwardMessage`]: ../methods/struct.ForwardMessage.html
    fn forward_to(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> ForwardMessage<'a> {
        self.bot().forward_message(chat_id, self.chat_id(), self.message_id())
    }
}
