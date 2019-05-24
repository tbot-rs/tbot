media_message! {
    struct Document {
        /// The document.
        document: types::Document,
        /// The caption of the document.
        caption: types::Text,
    } -> Bot::document

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}
