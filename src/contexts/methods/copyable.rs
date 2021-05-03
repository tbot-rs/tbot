use super::Message;
use crate::{methods::CopyMessage, types::parameters::ImplicitChatId};

/// Provides methods for copyable messages.
pub trait Copyable: Message {
    /// Copies this message to another chat.
    fn copy_to(&self, chat_id: impl ImplicitChatId) -> CopyMessage<'_> {
        self.bot()
            .copy_message(chat_id, self.chat().id, self.message_id())
    }
}
