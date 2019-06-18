media_message! {
    struct Venue {
        /// The venue.
        venue: types::Venue,
    } -> EventLoop::venue

    fn new() -> Self {
        Self { }
    }
}
