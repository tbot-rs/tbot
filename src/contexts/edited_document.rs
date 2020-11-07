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
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::edited_document

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}

impl fields::Document for EditedDocument {
    #[must_use]
    fn document(&self) -> &Document {
        &self.document
    }
}

impl Caption for EditedDocument {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for EditedDocument {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}
