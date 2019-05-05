media_message! {
    struct VenueContext {
        /// The venue.
        venue: types::Venue,
    } -> Bot::venue

    fn new() -> Self {
        Self { }
    }
}
