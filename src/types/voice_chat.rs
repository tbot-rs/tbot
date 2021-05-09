//! Types related to voice chats.

use super::User;
use serde::Deserialize;

/// Represents a service message about users invited to a voice chat.
///
/// See [`VoiceChatParticipantsInvited`] from Bot API docs.
///
/// [`VoiceChatParticipantsInvited`]: https://core.telegram.org/bots/api#voicechatparticipantsinvited
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct ParticipantsInvited {
    /// Users who were invited to the voice chat.
    pub users: Vec<User>,
}
