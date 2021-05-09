use crate::types::voice_chat;

message_base! {
    struct EndedVoiceChat {
        /// The duration of the voice chat in seconds.
        duration: u64,
    } -> EventLoop::ended_voice_chat

    fn new(ended: voice_chat::Ended,) -> Self {
        Self {
            duration: ended.duration,
        }
    }
}
