use crate::{contexts::fields::{self, AnyText}, types::message};

media_message! {
    struct Text {
        /// The text of the message.
        text: message::Text,
    } -> EventLoop::text

    fn new() -> Self {
        Self { }
    }
}

impl<C> fields::Text<C> for Text<C> {
    fn text(&self) -> &message::Text {
        &self.text
    }
}

impl<C> AnyText<C> for Text<C> {
    fn text(&self) -> &message::Text {
        &self.text
    }
}

