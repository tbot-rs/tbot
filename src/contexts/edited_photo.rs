use crate::{
    contexts::fields::{self, Album, AnyText, Caption},
    types::{message::Text, PhotoSize},
};

edited_message! {
    struct EditedPhoto {
        /// The photo.
        photo: Vec<PhotoSize>,
        /// The caption of the photo.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::edited_photo

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}

impl<C> fields::Photo<C> for EditedPhoto<C> {
    fn photo(&self) -> &[PhotoSize] {
        &self.photo[..]
    }
}

impl<C> Caption<C> for EditedPhoto<C> {
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for EditedPhoto<C> {
    fn text(&self) -> &Text {
        &self.caption
    }
}

impl<C> Album<C> for EditedPhoto<C> {
    fn media_group_id(&self) -> Option<&str> {
        self.media_group_id.as_ref().map(String::as_ref)
    }
}
