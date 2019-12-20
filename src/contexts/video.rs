use crate::{
    contexts::fields::{self, Album, AnyText, Caption},
    types::{self, message::Text},
};

media_message! {
    struct Video {
        /// The video.
        video: types::Video,
        /// The caption of the video.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::video

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}

impl<C> fields::Video<C> for Video<C> {
    #[must_use]
    fn video(&self) -> &types::Video {
        &self.video
    }
}

impl<C> Caption<C> for Video<C> {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for Video<C> {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

impl<C> Album<C> for Video<C> {
    #[must_use]
    fn media_group_id(&self) -> Option<&str> {
        self.media_group_id.as_ref().map(String::as_ref)
    }
}
