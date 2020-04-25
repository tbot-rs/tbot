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

impl fields::Photo for EditedPhoto {
    #[must_use]
    fn photo(&self) -> &[PhotoSize] {
        &self.photo[..]
    }
}

impl Caption for EditedPhoto {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for EditedPhoto {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

impl Album for EditedPhoto {
    #[must_use]
    fn media_group_id(&self) -> Option<&str> {
        self.media_group_id.as_ref().map(String::as_ref)
    }
}
