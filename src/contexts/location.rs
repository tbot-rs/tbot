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

impl<C> fields::Location<C> for Location<C> {
    fn location(&self) -> &types::Location {
        &self.location
    }
}
