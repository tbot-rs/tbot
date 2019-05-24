media_message! {
    struct Venue {
        /// The venue.
        venue: types::Venue,
    } -> Bot::venue

    fn new() -> Self {
        Self { }
    }
}
