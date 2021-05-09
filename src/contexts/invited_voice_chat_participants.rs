use crate::types::{voice_chat, User};

message_base! {
    struct InvitedVoiceChatParticipants {
        /// Users who were invited to the voice chat.
        users: Vec<User>,
    } -> EventLoop::invited_voice_chat_participants

    fn new(invited: voice_chat::ParticipantsInvited,) -> Self {
        Self {
            users: invited.users,
        }
    }
}
