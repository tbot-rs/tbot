edited_message! {
    struct EditedDocument {
        /// The document.
        document: types::Document,
        /// The caption of the document.
        caption: types::message::Text,
    } -> EventLoop::edited_document

    fn new(caption: types::message::Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}
