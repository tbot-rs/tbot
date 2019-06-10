use super::*;

/// Provides methods for pinnable messages.
pub trait Pinnable<'a>: ChatMethods<'a> {
    /// Pins this message.
    fn pin_this_message(&'a self) -> PinChatMessage<'a> {
        self.bot().pin_chat_message(self.chat_id(), self.message_id())
    }
}
