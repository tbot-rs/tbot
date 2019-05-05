media_message! {
    struct VideoContext {
        /// The video.
        video: types::Video,
        /// The video's caption.
        caption: String,
        /// Entities in the caption (links, formatting, etc).
        caption_entities: Vec<types::MessageEntity>,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> Bot::video

    fn new(caption: types::Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption.text,
            caption_entities: caption.entities,
            media_group_id: media_group_id,
        }
    }
}
