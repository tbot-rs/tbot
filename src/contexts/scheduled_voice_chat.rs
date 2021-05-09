use crate::types::voice_chat;

message_base! {
    struct ScheduledVoiceChat {
    /// Timestamp when the voice chat will be started.
        start_date: i64,
    } -> EventLoop::scheduled_voice_chat

    fn new(scheduled: voice_chat::Scheduled,) -> Self {
        Self {
            start_date: scheduled.start_date,
        }
    }
}
