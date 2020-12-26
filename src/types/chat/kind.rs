use crate::types::{chat, Message};
use is_macro::Is;

/// Represents the kind of a chat.
#[derive(Debug, PartialEq, Clone, Is)]
#[non_exhaustive]
pub enum Kind {
    /// The chat is private.
    #[non_exhaustive]
    Private {
        /// The username of the user.
        username: Option<String>,
        /// The first name of the user.
        first_name: String,
        /// The last name of the user.
        last_name: Option<String>,
        /// The bio of the other party in a private chat. Returned only
        /// in [`GetChat`].
        ///
        /// [`GetChat`]: crate::methods::GetChat
        bio: Option<String>,
    },
    /// The chat is a group.
    #[non_exhaustive]
    Group {
        /// The title of the group.
        title: String,
        /// The description of the group.
        description: Option<String>,
        /// The invite link of the group.
        invite_link: Option<String>,
        /// The pinned message of the group.
        pinned_message: Option<Box<Message>>,
        /// Default member permissions of the group. Returned only
        /// in [`GetChat`].
        ///
        /// [`GetChat`]: crate::methods::GetChat
        permissions: Option<chat::Permissions>,
    },
    /// The chat is a supergroup.
    #[non_exhaustive]
    Supergroup {
        /// The title of the supergroup.
        title: String,
        /// The username of the supergroup.
        username: Option<String>,
        /// The description of the supergroup.
        description: Option<String>,
        /// The invite link of the supergroup.
        invite_link: Option<String>,
        /// The pinned message of the supergroup.
        pinned_message: Option<Box<Message>>,
        /// The minimum allowed delay between messages in the supergroup.
        /// Returned only in [`GetChat`].
        ///
        /// [`GetChat`]: crate::methods::GetChat
        slow_mode_delay: Option<u64>,
        /// The name of the sticker set of the supergroup.
        sticker_set_name: Option<String>,
        /// `true` if the bot can set the sticker set of the supergroup.
        can_set_sticker_set: Option<bool>,
        /// Default member permissions of the supergroup.
        /// Returned only in [`GetChat`].
        ///
        /// [`GetChat`]: crate::methods::GetChat
        permissions: Option<chat::Permissions>,
        /// For supergroups, the location to which the supergroup is connected.
        /// Returned only in [`GetChat`].
        ///
        /// [`GetChat`]: crate::methods::GetChat
        location: Option<chat::Location>,
        /// If this supergroup is linked to a channel, this field contains
        /// the channel's ID. Returned only in [`GetChat`].
        ///
        /// [`GetChat`]: crate::methods::GetChat
        linked_chat_id: Option<chat::Id>,
    },
    /// The chat is a channel.
    #[non_exhaustive]
    Channel {
        /// The title of the channel.
        title: String,
        /// The username of the channel.
        username: Option<String>,
        /// The description of the channel.
        description: Option<String>,
        /// The invite link of the channel.
        invite_link: Option<String>,
        /// The pinned message of the channel.
        pinned_message: Option<Box<Message>>,
        /// If this channel has a discussion group, this field contains
        /// the group's ID. Returned only in [`GetChat`].
        ///
        /// [`GetChat`]: crate::methods::GetChat
        linked_chat_id: Option<chat::Id>,
    },
}
