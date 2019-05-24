media_message! {
    struct Text {
        /// The text of the message.
        text: types::Text,
    } -> Bot::text

    fn new() -> Self {
        Self { }
    }
}
