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

impl fields::Text for EditedText {
    #[must_use]
    fn text(&self) -> &Text {
        &self.text
    }
}

impl AnyText for EditedText {
    #[must_use]
    fn text(&self) -> &Text {
        &self.text
    }
}
