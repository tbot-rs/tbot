media_message! {
    struct Location {
        /// The location.
        location: types::Location,
    } -> EventLoop::location

    fn new() -> Self {
        Self { }
    }
}
