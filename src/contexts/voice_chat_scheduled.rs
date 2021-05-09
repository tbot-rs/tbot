use crate::types::voice_chat;

message_base! {
    struct VoiceChatScheduled {
    /// Timestamp when the voice chat will be started.
        start_date: i64,
    } -> EventLoop::voice_chat_scheduled

    fn new(scheduled: voice_chat::Scheduled,) -> Self {
        Self {
            start_date: scheduled.start_date,
        }
    }
}
