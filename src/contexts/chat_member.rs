use crate::{
    types::{chat, Chat, User},
    Bot,
};

common! {
    /// The context for [`chat_member`] handlers.
    ///
    /// [`chat_member`]: crate::EventLoop::chat_member
    struct ChatMember {
        /// The chat in which the change occured.
        chat: Chat,
        /// The user who caused the change.
        from: User,
        /// Timestamp when this change occured.
        date: i64,
        /// Previous information about the member.
        before: chat::Member,
        /// New information about the member.
        after: chat::Member,
        /// The invite link which the user used to join the chat.
        invite_link: Option<chat::InviteLink>,
    }
}

impl ChatMember {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Bot, update: chat::member::Updated) -> Self {
        Self {
            bot,
            chat: update.chat,
            from: update.from,
            date: update.date,
            before: update.before,
            after: update.after,
            invite_link: update.invite_link,
        }
    }
}
