use crate::types::Message;

message_base! {
    struct PinnedMessage {
        /// The pinned message.
        message: Message,
    } -> EventLoop::pinned_message

    fn new (message: Message,) -> Self {
        Self {
            message: message,
        }
    }
}
