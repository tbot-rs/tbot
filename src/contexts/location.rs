media_message! {
    struct LocationContext {
        /// The location.
        location: types::Location,
    } -> Bot::location

    fn new() -> Self {
        Self { }
    }
}
