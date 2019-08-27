use serde::{Deserialize, Serialize};

/// Describes actions that a non-administrator user is allowed to take in a
/// chat.
///
/// This struct is a representation of [`ChatPermissions`].
///
/// [`ChatPermissions`]: https://core.telegram.org/bots/api#chatpermissions
///
/// # Non-exhaustiveness
///
/// Users should not match this struct exhaustively. New fields added to it
/// are _not_ considered a breaking change per `tbot`'s
/// [breaking change policy].
///
/// [breaking change policy]: https://gitlab.com/SnejUgal/tbot/wikis/Breaking-changes
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, Default,
)]
pub struct Permissions {
    /// `true` if the user can send text messages, contacts, locations and
    /// venues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// `true` if the user can send audios, documents, photos, videos, video
    /// notes and voice notes. Implies `can_send_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,
    /// `true` if the user can send polls. Implies `can_send_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// `true` if the user can use inline bots and send animations, games and
    /// stickers. Implies `can_send_media_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// `true` if the user can add web page previews. Implies
    /// `can_send_media_messages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// `true` if the user can change the chat information. Ignored in public
    /// supergroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// `true` if the user can invite new users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// `true` is the user can pin messages. Ignored in public supegroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

impl Permissions {
    /// Constructs `Permissions` with all fields set to `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures if the user can send messages.
    pub fn can_send_messages(mut self, can_send: bool) -> Self {
        self.can_send_messages = Some(can_send);
        self
    }

    /// Configures if the user can send media messages.
    pub fn can_send_media_messages(mut self, can_send: bool) -> Self {
        self.can_send_media_messages = Some(can_send);
        self
    }

    /// Configures if the user can send polls.
    pub fn can_send_polls(mut self, can_send: bool) -> Self {
        self.can_send_polls = Some(can_send);
        self
    }

    /// Configures if the user can send other messages not covered by other
    /// permissions.
    pub fn can_send_other_messages(mut self, can_send: bool) -> Self {
        self.can_send_other_messages = Some(can_send);
        self
    }

    /// Configures if the user can add webpage previews.
    pub fn can_add_web_page_previews(mut self, can_add: bool) -> Self {
        self.can_add_web_page_previews = Some(can_add);
        self
    }

    /// Configures if the user can change chat information.
    pub fn can_change_info(mut self, can_change: bool) -> Self {
        self.can_change_info = Some(can_change);
        self
    }

    /// Configures if the user can invite new users to the chat.
    pub fn can_invite_users(mut self, can_invite: bool) -> Self {
        self.can_change_info = Some(can_invite);
        self
    }

    /// Configures if the user can pin messages.
    pub fn can_pin_messages(mut self, can_pin: bool) -> Self {
        self.can_change_info = Some(can_pin);
        self
    }
}
