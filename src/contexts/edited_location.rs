edited_message! {
    struct EditedLocation {
        /// The location.
        location: types::Location,
    } -> EventLoop::edited_location

    fn new() -> Self {
        Self { }
    }
}
