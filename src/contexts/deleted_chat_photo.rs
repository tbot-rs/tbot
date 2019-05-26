message_base! {
    struct DeletedChatPhoto { } -> Bot::deleted_chat_photo

    fn new() -> Self {
        Self {}
    }
}
