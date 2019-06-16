media_message! {
    struct Video {
        /// The video.
        video: types::Video,
        /// The caption of the video.
        caption: types::message::Text,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> Bot::video

    fn new(caption: types::message::Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}
