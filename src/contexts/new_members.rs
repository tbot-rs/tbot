use crate::types::User;

message_base! {
    struct NewMembers {
        /// The new members.
        members: Vec<User>,
    } -> EventLoop::new_members

    fn new(members: Vec<User>,) -> Self {
        Self {
            members: members,
        }
    }
}
