use crate::{
    contexts::fields::{self, AnyText},
    types::message,
};

media_message! {
    struct Text {
        /// The text of the message.
        text: message::Text,
    } -> EventLoop::text

    fn new() -> Self {
        Self { }
    }
}

impl fields::Text for Text {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.text
    }
}

impl AnyText for Text {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.text
    }
}
