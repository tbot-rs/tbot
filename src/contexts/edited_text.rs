use crate::types::message::Text;

edited_message! {
    struct EditedText {
        /// The text of the message.
        text: Text,
    } -> EventLoop::edited_text

    fn new() -> Self {
        Self { }
    }
}
