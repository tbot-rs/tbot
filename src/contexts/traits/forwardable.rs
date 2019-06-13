use super::*;

/// Provides methods for forwardable messages.
pub trait Forwardable<'a, C: 'static>: ChatMethods<'a, C> {
    /// Forwards this message to another chat.
    fn forward_to(
        &'a self,
        chat_id: impl Into<types::ChatId<'a>>,
    ) -> ForwardMessage<'a, C> {
        self.bot().forward_message(chat_id, self.chat_id(), self.message_id())
    }
}
