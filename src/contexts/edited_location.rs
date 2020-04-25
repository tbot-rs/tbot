use crate::{contexts::fields, types::Location};

edited_message! {
    struct EditedLocation {
        /// The location.
        location: Location,
    } -> EventLoop::edited_location

    fn new() -> Self {
        Self { }
    }
}

impl fields::Location for EditedLocation {
    #[must_use]
    fn location(&self) -> &Location {
        &self.location
    }
}
