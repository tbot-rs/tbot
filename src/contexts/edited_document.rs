use crate::types::{message::Text, Document};

edited_message! {
    struct EditedDocument {
        /// The document.
        document: Document,
        /// The caption of the document.
        caption: Text,
    } -> EventLoop::edited_document

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}
