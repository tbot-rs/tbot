media_message! {
    struct VideoContext {
        /// The video.
        video: types::Video,
        /// The caption of the video.
        caption: types::Text,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> Bot::video

    fn new(caption: types::Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}
