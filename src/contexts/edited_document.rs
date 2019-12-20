use crate::{
    contexts::fields::{self, AnyText, Caption},
    types::{message::Text, Document},
};

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

impl<C> fields::Document<C> for EditedDocument<C> {
    #[must_use]
    fn document(&self) -> &Document {
        &self.document
    }
}

impl<C> Caption<C> for EditedDocument<C> {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for EditedDocument<C> {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}
