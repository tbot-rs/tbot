use crate::types::User;
use serde::Deserialize;

/// Represents an invite link of a chat.
///
/// See [`ChatInviteLink`] from Bot API docs.
///
/// [`ChatInviteLink`]: https://core.telegram.org/bots/api#chatinvitelink
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct InviteLink {
    /// The invite link itself.
    #[serde(rename = "invite_link")]
    pub link: String,
    /// The user who created this invite link.
    pub creator: User,
    /// `true` if this link is the primary one.
    pub is_primary: bool,
    /// `true` if this link has been revoked.
    pub is_revoked: bool,
    /// Timestamp when this link expires.
    pub expire_date: Option<i64>,
    /// Maximum amount of users that can be chat members at the same time
    /// when joining via this invite link. In range `1..100_000`.
    pub member_limit: Option<u32>,
}
