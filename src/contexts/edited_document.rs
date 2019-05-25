edited_message! {
    struct EditedDocument {
        /// The document.
        document: types::Document,
        /// The caption of the document.
        caption: types::Text,
    } -> Bot::edited_document

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}
