message_base! {
    struct NewChatTitle {
        /// The title.
        title: String,
    } -> Bot::new_chat_title

    fn new(title: String,) -> Self {
        Self {
            title: title,
        }
    }
}
