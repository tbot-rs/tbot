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

/// Represents a service message about an ended voice chat.
///
/// See [`VoiceChatEnded`] from Bot API docs.
///
/// [`VoiceChatEnded`]: https://core.telegram.org/bots/api#voicechatended
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
#[non_exhaustive]
pub struct Ended {
    /// The duration of the voice chat in seconds.
    pub duration: u64,
}

/// Represents a service message about a scheduled voice chat.
///
/// See [`VoiceChatScheduled`] from Bot API docs.
///
/// [`VoiceChatScheduled`]: https://core.telegram.org/bots/api#voicechatscheduled
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
#[non_exhaustive]
pub struct Scheduled {
    /// Timestamp when the voice chat will be started.
    pub start_date: i64,
}
