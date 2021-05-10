use crate::{
    types::{chat, Chat, User},
    Bot,
};

common! {
    /// The context for [`my_chat_member`] handlers.
    ///
    /// [`my_chat_member`]: crate::EventLoop::my_chat_member
    struct MyChatMember {
        /// The chat in which the change occured.
        chat: Chat,
        /// The user who caused the change.
        from: User,
        /// Timestamp when this change occured.
        date: i64,
        /// Previous information about the bot's member status.
        before: chat::Member,
        /// New information about the bot's member status.
        after: chat::Member,
    }
}

impl MyChatMember {
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(bot: Bot, update: chat::member::Updated) -> Self {
        Self {
            bot,
            chat: update.chat,
            from: update.from,
            date: update.date,
            before: update.before,
            after: update.after,
        }
    }
}
