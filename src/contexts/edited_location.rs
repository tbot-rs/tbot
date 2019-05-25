edited_message! {
    struct EditedLocation {
        /// The location.
        location: types::Location,
    } -> Bot::edited_location

    fn new() -> Self {
        Self { }
    }
}
