use crate::types::message;

message_base! {
    struct ChangedAutoDeleteTimer {
        /// The new auto-delete timer value.
        auto_delete_time: u64,
    } -> EventLoop::changed_auto_delete_timer

    fn new(change: message::AutoDeleteTimerChanged,) -> Self {
        Self {
            auto_delete_time: change.auto_delete_time,
        }
    }
}
