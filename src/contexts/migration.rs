common! {
    /// Context for the [`migration`][handler] handler.
    ///
    /// This context does not provide the `chat` field to prevent possible
    /// wrong expectations for `chat.id` to be the old ID.
    ///
    /// [handler]: ../struct.Bot.html#method.migration
    struct Migration {
        /// ID of the message.
        message_id: u32,
        /// The sender of the message.
        from: types::User,
        /// The time the message was sent at.
        date: i64,
        /// The old ID of the group.
        old_id: i64,
        /// The new ID of the group.
        new_id: i64,
    }
}

impl Migration {
    pub(crate) fn new(bot: Arc<MockBot>, data: types::MessageData, old_id: i64,) -> Self {
        Self {
            bot,
            message_id: data.id,
            from: data.from.expect("\n[tbot] Expected `from` to exist on a `migration_from_chat_id` update\n"),
            date: data.date,
            old_id,
            new_id: data.chat.id,
        }
    }
}
