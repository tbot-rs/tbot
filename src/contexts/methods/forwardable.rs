use super::Message;
use crate::{methods::ForwardMessage, types::parameters::ImplicitChatId};

/// Provides methods for forwardable messages.
pub trait Forwardable: Message {
    /// Forwards this message to another chat.
    fn forward_to(
        &self,
        chat_id: impl ImplicitChatId,
    ) -> ForwardMessage<'_> {
        self.bot()
            .forward_message(chat_id, self.chat().id, self.message_id())
    }
}
