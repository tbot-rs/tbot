use crate::{contexts::fields, types};

media_message! {
    struct Location {
        /// The location.
        location: types::Location,
    } -> EventLoop::location

    fn new() -> Self {
        Self { }
    }
}

impl fields::Location for Location {
    #[must_use]
    fn location(&self) -> &types::Location {
        &self.location
    }
}
