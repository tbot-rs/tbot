edited_message! {
    struct EditedTextContext {
        /// The text of the message.
        text: types::Text,
    } -> Bot::edited_text

    fn new() -> Self {
        Self { }
    }
}
