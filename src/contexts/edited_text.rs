edited_message! {
    struct EditedText {
        /// The text of the message.
        text: types::message::Text,
    } -> EventLoop::edited_text

    fn new() -> Self {
        Self { }
    }
}
