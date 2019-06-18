use crate::types::{Video, message::Text};

edited_message! {
    struct EditedVideo {
        /// The video.
        video: Video,
        /// The caption of the video.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> EventLoop::edited_video

    fn new(caption: Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}
