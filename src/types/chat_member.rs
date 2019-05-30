use super::*;

/// Represents the status of a member.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MemberStatus {
    /// The user is the creator of the chat.
    Creator,
    /// The user is an administrator of the chat.
    Administator {
        /// Whether the bot can edit this admin's rights.
        can_be_edited: bool,
        /// Whether the admin can change the group's info.
        can_change_info: bool,
        /// Whether the admin can post messages (channels only).
        can_post_messages: Option<bool>,
        /// Whether the admin can edit messages (channels only).
        can_edit_messages: Option<bool>,
        /// Whether the admin can delete messages.
        can_delete_messages: bool,
        /// Whether the admin can invite users.
        can_invite_users: bool,
        /// Whether the admin can restruct users.
        can_restrict_members: bool,
        /// Whether the admin can pin messages.
        can_pin_messages: bool,
        /// Whether the admin can promote members.
        can_promote_members: bool,
    },
    /// The user is a member of the chat.
    Member,
    /// The user is restricted in the chat.
    Restricted {
        /// Time when the restriction will be lifted.
        until_date: Option<i64>,
        /// Whether the user is a member of the chat.
        is_member: bool,
        /// Whether the user can send messages.
        can_send_mesages: bool,
        /// Whether the user can send media messages.
        can_send_media_messages: bool,
        /// Whether the user can send other messages, such as games.
        can_send_other_messages: bool,
        /// Whehter the user can semd messages with link previews.
        can_add_web_page_previews: bool,
    },
    /// The user left the chat.
    Left,
    /// The user was kicked out of the chat.
    Kicked {
        /// Time when the restriction will be lifted.
        until_date: Option<i64>,
    },
}

/// Represents a [`ChatMember`].
///
/// [`ChatMember`]: https://core.telegram.org/bots/api#chatmember
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ChatMember {
    /// Information about the member.
    pub user: User,
    /// Status of the member.
    pub status: MemberStatus,
}

const USER: &str = "user";
const STATUS: &str = "status";
const UNTIL_DATE: &str = "until_date";
const CAN_BE_EDITED: &str = "can_be_edited";
const CAN_CHANGE_INFO: &str = "can_change_info";
const CAN_POST_MESSAGES: &str = "can_post_messages";
const CAN_EDIT_MESSAGES: &str = "can_edit_messages";
const CAN_DELETE_MESSAGES: &str = "can_delete_messages";
const CAN_INVITE_USERS: &str = "can_invite_users";
const CAN_RESTRICT_MEMBERS: &str = "can_restrict_members";
const CAN_PIN_MESSAGES: &str = "can_pin_messages";
const CAN_PROMOTE_MEMBERS: &str = "can_promote_members";
const IS_MEMBER: &str = "is_member";
const CAN_SEND_MESSAGES: &str = "can_send_messages";
const CAN_SEND_MEDIA_MESSAGES: &str = "can_send_media_messages";
const CAN_SEND_OTHER_MESSAGES: &str = "can_send_other_messages";
const CAN_ADD_WEB_PAGE_PREVIEWS: &str = "can_add_web_page_previews";

const CREATOR: &str = "creator";
const ADMINISTRATOR: &str = "administrator";
const MEMBER: &str = "member";
const RESTRICTED: &str = "restricted";
const LEFT: &str = "left";
const KICKED: &str = "kicked";

struct ChatMemberVisitor;

impl<'v> serde::de::Visitor<'v> for ChatMemberVisitor {
    type Value = ChatMember;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct ChatMember")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut user = None;
        let mut status = None;
        let mut until_date = None;
        let mut can_be_edited = None;
        let mut can_change_info = None;
        let mut can_post_messages = None;
        let mut can_edit_messages = None;
        let mut can_delete_messages = None;
        let mut can_invite_users = None;
        let mut can_restrict_members = None;
        let mut can_pin_messages = None;
        let mut can_promote_members = None;
        let mut is_member = None;
        let mut can_send_messages = None;
        let mut can_send_media_messages = None;
        let mut can_send_other_messages = None;
        let mut can_add_web_page_previews = None;

        while let Some(key) = map.next_key()? {
            match key {
                USER => user = Some(map.next_value()?),
                STATUS => status = Some(map.next_value()?),
                UNTIL_DATE => until_date = Some(map.next_value()?),
                CAN_BE_EDITED => can_be_edited = Some(map.next_value()?),
                CAN_CHANGE_INFO => can_change_info = Some(map.next_value()?),
                CAN_POST_MESSAGES => {
                    can_post_messages = Some(map.next_value()?)
                }
                CAN_EDIT_MESSAGES => {
                    can_edit_messages = Some(map.next_value()?)
                }
                CAN_DELETE_MESSAGES => {
                    can_delete_messages = Some(map.next_value()?)
                }
                CAN_INVITE_USERS => can_invite_users = Some(map.next_value()?),
                CAN_RESTRICT_MEMBERS => {
                    can_restrict_members = Some(map.next_value()?)
                }
                CAN_PIN_MESSAGES => can_pin_messages = Some(map.next_value()?),
                CAN_PROMOTE_MEMBERS => {
                    can_promote_members = Some(map.next_value()?)
                }
                IS_MEMBER => is_member = Some(map.next_value()?),
                CAN_SEND_MESSAGES => {
                    can_send_messages = Some(map.next_value()?)
                }
                CAN_SEND_MEDIA_MESSAGES => {
                    can_send_media_messages = Some(map.next_value()?)
                }
                CAN_SEND_OTHER_MESSAGES => {
                    can_send_other_messages = Some(map.next_value()?)
                }
                CAN_ADD_WEB_PAGE_PREVIEWS => {
                    can_add_web_page_previews = Some(map.next_value()?)
                }
                _ => {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let status = match &status {
            Some(CREATOR) => MemberStatus::Creator,
            Some(ADMINISTRATOR) => MemberStatus::Administator {
                can_be_edited: can_be_edited.ok_or_else(|| {
                    serde::de::Error::missing_field(CAN_BE_EDITED)
                })?,
                can_change_info: can_change_info.ok_or_else(|| {
                    serde::de::Error::missing_field(CAN_CHANGE_INFO)
                })?,
                can_post_messages,
                can_edit_messages,
                can_delete_messages: can_delete_messages.ok_or_else(|| {
                    serde::de::Error::missing_field(CAN_DELETE_MESSAGES)
                })?,
                can_invite_users: can_invite_users.ok_or_else(|| {
                    serde::de::Error::missing_field(CAN_INVITE_USERS)
                })?,
                can_restrict_members: can_restrict_members.ok_or_else(
                    || serde::de::Error::missing_field(CAN_RESTRICT_MEMBERS),
                )?,
                can_pin_messages: can_pin_messages.ok_or_else(|| {
                    serde::de::Error::missing_field(CAN_PIN_MESSAGES)
                })?,
                can_promote_members: can_promote_members.ok_or_else(|| {
                    serde::de::Error::missing_field(CAN_PROMOTE_MEMBERS)
                })?,
            },
            Some(MEMBER) => MemberStatus::Member,
            Some(RESTRICTED) => MemberStatus::Restricted {
                until_date,
                is_member: is_member.ok_or_else(|| {
                    serde::de::Error::missing_field(IS_MEMBER)
                })?,
                can_send_mesages: can_send_messages.ok_or_else(|| {
                    serde::de::Error::missing_field(CAN_SEND_MESSAGES)
                })?,
                can_send_media_messages: can_send_media_messages.ok_or_else(
                    || serde::de::Error::missing_field(CAN_SEND_MEDIA_MESSAGES),
                )?,
                can_send_other_messages: can_send_other_messages.ok_or_else(
                    || serde::de::Error::missing_field(CAN_SEND_OTHER_MESSAGES),
                )?,
                can_add_web_page_previews: can_add_web_page_previews
                    .ok_or_else(|| {
                        serde::de::Error::missing_field(
                            CAN_ADD_WEB_PAGE_PREVIEWS,
                        )
                    })?,
            },
            Some(LEFT) => MemberStatus::Left,
            Some(KICKED) => MemberStatus::Kicked {
                until_date,
            },
            Some(unknown_status) => {
                return Err(serde::de::Error::unknown_variant(
                    unknown_status,
                    &[CREATOR, ADMINISTRATOR, MEMBER, RESTRICTED, LEFT, KICKED],
                ))
            }
            None => return Err(serde::de::Error::missing_field(STATUS)),
        };

        Ok(ChatMember {
            user: user.ok_or_else(|| serde::de::Error::missing_field(USER))?,
            status,
        })
    }
}

impl<'de> Deserialize<'de> for ChatMember {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "ChatMember",
            &[
                USER,
                STATUS,
                UNTIL_DATE,
                CAN_BE_EDITED,
                CAN_CHANGE_INFO,
                CAN_POST_MESSAGES,
                CAN_EDIT_MESSAGES,
                CAN_DELETE_MESSAGES,
                CAN_INVITE_USERS,
                CAN_RESTRICT_MEMBERS,
                CAN_PIN_MESSAGES,
                CAN_PROMOTE_MEMBERS,
                IS_MEMBER,
                CAN_SEND_MESSAGES,
                CAN_SEND_MEDIA_MESSAGES,
                CAN_SEND_OTHER_MESSAGES,
                CAN_ADD_WEB_PAGE_PREVIEWS,
            ],
            ChatMemberVisitor,
        )
    }
}
