message_base! {
    struct DeletedChatPhoto { } -> EventLoop::deleted_chat_photo

    fn new() -> Self {
        Self {}
    }
}
