media_message! {
    struct Poll {
        /// The poll.
        poll: types::Poll,
    } -> Bot::poll

    fn new() -> Self {
        Self { }
    }
}
