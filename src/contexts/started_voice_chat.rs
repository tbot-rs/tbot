message_base! {
    struct StartedVoiceChat {} -> EventLoop::started_voice_chat

    fn new() -> Self {
        Self {}
    }
}
