use crate::{contexts::fields::{Caption, AnyText}, types::{message::Text, Document}};

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

impl<C> Caption<C> for EditedDocument<C> {
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for EditedDocument<C> {
    fn text(&self) -> &Text {
        &self.caption
    }
}

