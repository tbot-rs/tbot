message_base! {
    struct PinnedMessage {
        /// The pinned message.
        message: types::Message,
    } -> EventLoop::pinned_message

    fn new (message: types::Message,) -> Self {
        Self {
            message: message,
        }
    }
}
