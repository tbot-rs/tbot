use crate::types::{self, message::Text};

media_message! {
    struct Document {
        /// The document.
        document: types::Document,
        /// The caption of the document.
        caption: Text,
    } -> EventLoop::document

    fn new(caption: Text,) -> Self {
        Self {
            caption: caption,
        }
    }
}
