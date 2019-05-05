media_message! {
    struct DocumentContext {
        /// The document.
        document: types::Document,
        /// The document's caption.
        caption: String,
        /// Entities in the caption (links, formatting, etc).
        caption_entities: Vec<types::MessageEntity>,
    } -> Bot::document

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption.text,
            caption_entities: caption.entities,
        }
    }
}
