use crate::{
    contexts::fields::{self, AnyText},
    types::message::Text,
};

edited_message! {
    struct EditedText {
        /// The text of the message.
        text: Text,
    } -> EventLoop::edited_text

    fn new() -> Self {
        Self { }
    }
}

impl<C> fields::Text<C> for EditedText<C> {
    fn text(&self) -> &Text {
        &self.text
    }
}

impl<C> AnyText<C> for EditedText<C> {
    fn text(&self) -> &Text {
        &self.text
    }
}
