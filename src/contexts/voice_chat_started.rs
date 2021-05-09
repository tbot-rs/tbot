message_base! {
    struct VoiceChatStarted {} -> EventLoop::voice_chat_started

    fn new() -> Self {
        Self {}
    }
}
