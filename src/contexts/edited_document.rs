edited_message! {
    struct EditedDocument {
        /// The document.
        document: types::Document,
        /// The caption of the document.
        caption: types::message::Text,
    } -> Bot::edited_document

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}
