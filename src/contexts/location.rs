media_message! {
    struct Location {
        /// The location.
        location: types::Location,
    } -> Bot::location

    fn new() -> Self {
        Self { }
    }
}
