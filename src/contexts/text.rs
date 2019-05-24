media_message! {
    struct TextContext {
        /// The text of the message.
        text: types::Text,
    } -> Bot::text

    fn new() -> Self {
        Self { }
    }
}
