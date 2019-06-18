message_base! {
    struct NewMembers {
        /// The new members.
        members: Vec<types::User>,
    } -> EventLoop::new_members

    fn new(members: Vec<types::User>,) -> Self {
        Self {
            members: members,
        }
    }
}
