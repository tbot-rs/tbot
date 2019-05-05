media_message! {
    struct AudioContext {
        /// The audio.
        audio: types::Audio,
        /// The audio's caption.
        caption: String,
        /// Entities in the caption (links, formatting, etc).
        caption_entities: Vec<types::MessageEntity>,
    } -> Bot::audio

    fn new(caption: types::Text,) -> Self {
        Self {
            caption: caption.text,
            caption_entities: caption.entities,
        }
    }
}
