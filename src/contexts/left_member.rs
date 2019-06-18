use crate::types::User;

message_base! {
    struct LeftMember {
        /// The left member.
        member: User,
    } -> EventLoop::left_member

    fn new(member: User,) -> Self {
        Self {
            member: member,
        }
    }
}
