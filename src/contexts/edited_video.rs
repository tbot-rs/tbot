use crate::{contexts::fields::{Caption, AnyText}, types::{message::Text, Video}};

edited_message! {
    struct EditedVideo {
        /// The video.
        video: Video,
        /// The caption of the video.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::edited_video

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}

impl<C> Caption<C> for EditedVideo<C> {
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl<C> AnyText<C> for EditedVideo<C> {
    fn text(&self) -> &Text {
        &self.caption
    }
}
