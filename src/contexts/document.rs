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

impl<C> fields::Document<C> for Document<C> {
    #[must_use]
    fn document(&self) -> &types::Document {
        &self.document
    }
}

impl<C> Caption<C> for Document<C> {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for Document<C> {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}
