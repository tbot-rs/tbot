message_base! {
    struct LeftMember {
        /// The left member.
        member: types::User,
    } -> EventLoop::left_member

    fn new(member: types::User,) -> Self {
        Self {
            member: member,
        }
    }
}
