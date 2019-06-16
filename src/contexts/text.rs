media_message! {
    struct Text {
        /// The text of the message.
        text: types::message::Text,
    } -> Bot::text

    fn new() -> Self {
        Self { }
    }
}
