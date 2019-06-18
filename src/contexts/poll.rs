media_message! {
    struct Poll {
        /// The poll.
        poll: types::Poll,
    } -> EventLoop::poll

    fn new() -> Self {
        Self { }
    }
}
