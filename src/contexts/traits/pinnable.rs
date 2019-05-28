use super::*;

/// Contains methods for pinnable messages.
pub trait Pinnable<'a>: ChatMethods<'a> {
    /// Constructs a new [`PinChatMessage`] inferring the token, the chat ID and
    /// the message ID.
    ///
    /// [`PinChatMessage`]: ../methods/struct.PinChatMessage.html
    fn pin_this_message(&'a self) -> PinChatMessage<'a> {
        self.bot().pin_chat_message(self.chat_id(), self.message_id())
    }
}
