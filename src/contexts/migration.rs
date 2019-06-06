message_base! {
    struct Migration {
        /// The old ID of the group.
        old_id: i64,
    } -> Bot::migration

    fn new(old_id: i64,) -> Self {
        Self {
            old_id: old_id,
        }
    }
}
