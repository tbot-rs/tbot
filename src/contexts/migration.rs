use crate::types::chat;

message_base! {
    struct Migration {
        /// The old ID of the group.
        old_id: chat::Id,
    } -> EventLoop::migration

    fn new(old_id: chat::Id,) -> Self {
        Self {
            old_id: old_id,
        }
    }
}
