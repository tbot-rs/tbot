use crate::{
    contexts::fields::{self, Album, AnyText, Caption},
    types::{message::Text, PhotoSize},
};

media_message! {
    struct Photo {
        /// The photo.
        photo: Vec<PhotoSize>,
        /// The caption of the photo.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::photo

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}

impl<C> fields::Photo<C> for Photo<C> {
    #[must_use]
    fn photo(&self) -> &[PhotoSize] {
        &self.photo[..]
    }
}

impl<C> Caption<C> for Photo<C> {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for Photo<C> {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

impl<C> Album<C> for Photo<C> {
    #[must_use]
    fn media_group_id(&self) -> Option<&str> {
        self.media_group_id.as_ref().map(String::as_ref)
    }
}
