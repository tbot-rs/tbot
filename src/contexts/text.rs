use crate::types::message;

media_message! {
    struct Text {
        /// The text of the message.
        text: message::Text,
    } -> EventLoop::text

    fn new() -> Self {
        Self { }
    }
}
