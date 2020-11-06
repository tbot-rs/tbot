use crate::{
    contexts::fields::{self, AnyText},
    types::message,
};

edited_message! {
    struct EditedCommand {
        /// The text of the message.
        text: message::Text,
        /// The command which triggered the handler.
        command: String,
    } -> EventLoop::text

    fn new(command: String,) -> Self {
        Self {
            command: command,
        }
    }
}

impl fields::Text for EditedCommand {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.text
    }
}

impl AnyText for EditedCommand {
    #[must_use]
    fn text(&self) -> &message::Text {
        &self.text
    }
}
