media_message! {
    struct Document {
        /// The document.
        document: types::Document,
        /// The caption of the document.
        caption: types::message::Text,
    } -> Bot::document

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}
