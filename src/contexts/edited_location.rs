use crate::types::Location;

edited_message! {
    struct EditedLocation {
        /// The location.
        location: Location,
    } -> EventLoop::edited_location

    fn new() -> Self {
        Self { }
    }
}
