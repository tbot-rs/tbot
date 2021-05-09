use crate::types::{voice_chat, User};

message_base! {
    struct VoiceChatParticipantsInvited {
        /// Users who were invited to the voice chat.
        users: Vec<User>,
    } -> EventLoop::voice_chat_participants_invited

    fn new(invited: voice_chat::ParticipantsInvited,) -> Self {
        Self {
            users: invited.users,
        }
    }
}
