media_message! {
    struct PollContext {
        /// The poll.
        poll: types::Poll,
    } -> Bot::poll

    fn new() -> Self {
        Self { }
    }
}
