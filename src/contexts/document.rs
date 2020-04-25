use crate::{
    contexts::fields::{self, AnyText, Caption},
    types::{self, message::Text},
};

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

impl fields::Document for Document {
    #[must_use]
    fn document(&self) -> &types::Document {
        &self.document
    }
}

impl Caption for Document {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for Document {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}
