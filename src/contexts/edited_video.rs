use crate::{
    contexts::fields::{self, Album, AnyText, Caption},
    types::{message::Text, Video},
};

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

impl fields::Video for EditedVideo {
    #[must_use]
    fn video(&self) -> &Video {
        &self.video
    }
}

impl Caption for EditedVideo {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for EditedVideo {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

impl Album for EditedVideo {
    #[must_use]
    fn media_group_id(&self) -> Option<&str> {
        self.media_group_id.as_ref().map(String::as_ref)
    }
}
