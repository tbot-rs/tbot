use crate::types::voice_chat;

message_base! {
    struct VoiceChatEnded {
        /// The duration of the voice chat in seconds.
        duration: u64,
    } -> EventLoop::voice_chat_ended

    fn new(ended: voice_chat::Ended,) -> Self {
        Self {
            duration: ended.duration,
        }
    }
}
