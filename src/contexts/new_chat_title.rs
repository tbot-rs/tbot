message_base! {
    struct NewChatTitle {
        /// The title.
        title: String,
    } -> EventLoop::new_chat_title

    fn new(title: String,) -> Self {
        Self {
            title: title,
        }
    }
}
